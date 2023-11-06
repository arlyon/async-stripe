use std::collections::HashMap;
use std::fs::File;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use lazy_static::lazy_static;
use petgraph::algo::is_cyclic_directed;
use petgraph::Direction;
use serde::{Deserialize, Serialize};
use tracing::{error, trace};

use crate::components::Components;
use crate::graph::ComponentGraph;
use crate::types::ComponentPath;

/// Paths for components required to live in the `stripe_types` crate. Adding `event` is
/// used for webhooks
const PATHS_IN_TYPES: &[&str] = &["event"];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Crate {
    Core,
    Types,
    Misc,
    Connect,
    Terminal,
    Checkout,
    Treasury,
    Product,
    Billing,
    Payment,
    Issuing,
    Fraud,
}

impl Crate {
    pub const fn all() -> &'static [Self] {
        use Crate::*;
        &[
            Core, Types, Misc, Connect, Terminal, Checkout, Treasury, Product, Billing, Payment,
            Issuing, Fraud,
        ]
    }

    pub fn generate_to(self) -> String {
        let out_path = self.generated_out_path();
        if self == Self::Types {
            out_path
        } else {
            format!("{out_path}/src")
        }
    }

    pub fn generated_out_path(self) -> String {
        if self == Self::Types {
            "stripe_types".into()
        } else {
            format!("crates/{}", self.name())
        }
    }

    pub fn name(self) -> String {
        format!("stripe_{}", self.suffix())
    }

    pub const fn suffix(self) -> &'static str {
        use Crate::*;
        match self {
            Core => "core",
            Types => "types",
            Misc => "misc",
            Connect => "connect",
            Terminal => "terminal",
            Checkout => "checkout",
            Treasury => "treasury",
            Product => "product",
            Billing => "billing",
            Payment => "payment",
            Issuing => "issuing",
            Fraud => "fraud",
        }
    }
}

#[derive(Deserialize)]
struct CrateMap {
    components: HashMap<Crate, Vec<String>>,
    packages: HashMap<String, Crate>,
}

fn load_crate_map() -> anyhow::Result<CrateMap> {
    let loaded = serde_json::from_reader(File::open("crate_map.json")?)?;
    Ok(loaded)
}

lazy_static! {
    static ref CRATE_MAP: CrateMap = load_crate_map().expect("Could not load crate map");
}

pub fn validate_crate_map(components: &Components) -> anyhow::Result<()> {
    for members in CRATE_MAP.components.values() {
        for name in members {
            if !components.components.contains_key(name.as_str()) {
                bail!("Crate map includes unrecognized {name}. Maybe it is misspelled?");
            }
        }
    }
    Ok(())
}

pub fn maybe_infer_crate(path: &str, package: Option<&str>) -> Option<Crate> {
    // Make sure deleted variants end up in the same place as the non-deleted version
    let path = path.trim_start_matches("deleted_");
    if let Some(package) = package {
        let krate = CRATE_MAP.packages.get(package).unwrap_or_else(|| {
            panic!("Package {} requires a mapping", package);
        });
        return Some(*krate);
    }
    for krate in Crate::all() {
        let Some(members) = CRATE_MAP.components.get(krate) else {
            continue;
        };
        if members.iter().any(|c| c == path) {
            return Some(*krate);
        }
    }
    None
}

impl Components {
    pub fn infer_all_crate_assignments(&mut self) -> anyhow::Result<()> {
        // If a component includes requests that have URLs building off another component,
        // place with that component. This automates determinations like `external_account`
        // ending up in the same crate as `account` since all its requests start with `/account`.
        let mut new_assignments: IndexMap<ComponentPath, Crate> = IndexMap::new();
        for (path, component) in &self.components {
            let krate = component.krate();
            if krate.is_some() {
                continue;
            }
            let this_obj = self.get(path);
            let words = path.split('_');
            let first_two = words.take(2).collect::<Vec<_>>().join("_");
            let first_word = path.split_once('_').map(|f| f.0);
            for (other_path, other_component) in &self.components {
                let Some(krate) = other_component.krate() else {
                    continue;
                };
                if this_obj.is_nested_resource_of(other_component) {
                    new_assignments.insert(path.clone(), krate.base());
                    break;
                }

                if other_path.as_ref() == first_two {
                    new_assignments.insert(path.clone(), krate.base());
                    break;
                }
                if first_word == Some(other_path.as_ref()) {
                    new_assignments.insert(path.clone(), krate.base());
                    break;
                }
            }
        }
        trace!("Inferred based on naming: {new_assignments:#?} ");
        for (path, krate) in new_assignments {
            self.get_mut(&path).assign_crate(krate);
        }

        infer_crates_using_deps(self, Self::maybe_infer_crate_by_what_depends_on_it);
        infer_crates_using_deps(self, Self::maybe_infer_crate_by_deps);
        self.assign_uninferrable_crates();
        self.ensure_no_missing_crates()?;
        self.assign_paths_required_to_live_in_types_crate();
        self.validate_crate_assignment()
    }

    fn assign_uninferrable_crates(&mut self) {
        let uninferrable = [
            ("linked_account_options_us_bank_account", Crate::Core),
            ("three_d_secure_details", Crate::Types),
            ("radar_radar_options", Crate::Types),
        ];
        for (path, krate) in uninferrable {
            let comp = self.components.get_mut(path).expect("Component not found");
            if comp.krate().is_some() {
                panic!(
                    "Component {} already had crate {:?}. Cannot assign {:?}",
                    path,
                    comp.krate().unwrap(),
                    krate
                );
            }
            comp.assign_crate(krate);
        }
    }

    fn ensure_no_missing_crates(&self) -> anyhow::Result<()> {
        let missing_crates = self
            .components
            .iter()
            .filter(|(_, comp)| comp.krate().is_none())
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        if !missing_crates.is_empty() {
            let graph = self.gen_component_dep_graph();
            for missing in &missing_crates {
                let depended_on =
                    graph.neighbors_directed(missing, Direction::Incoming).collect::<Vec<_>>();
                let depended_crates =
                    depended_on.iter().map(|m| self.get(m).krate()).collect::<IndexSet<_>>();
                error!("Could not infer crate for {missing}. Depended on by components {depended_on:?}, crates {depended_crates:?}")
            }
            bail!("Some components could not have their crate inferred: {:#?}", missing_crates);
        }

        Ok(())
    }

    fn validate_crate_assignment(&self) -> anyhow::Result<()> {
        let graph = self.gen_crate_dep_graph();
        let deps_for_types_crate = graph.neighbors(Crate::Types).collect::<Vec<_>>();
        if !deps_for_types_crate.is_empty() {
            bail!(
                "Types crate should not have dependencies, but has dependencies {:#?}!",
                deps_for_types_crate
            );
        }
        if is_cyclic_directed(&graph) {
            bail!("Crate dependency graph contains a cycle!");
        }
        let requests_in_types = self
            .components
            .iter()
            .filter(|(_, comp)| {
                comp.krate_unwrapped().base() == Crate::Types && !comp.requests.is_empty()
            })
            .map(|(path, _)| path)
            .collect::<Vec<_>>();
        if !requests_in_types.is_empty() {
            bail!("Components have requests, not allowed in types crate: {requests_in_types:#?}");
        }
        Ok(())
    }

    fn assign_paths_required_to_live_in_types_crate(&mut self) {
        let mut required = vec![];
        loop {
            let graph = self.gen_component_dep_graph();
            for (path, component) in &self.components {
                if component.krate_unwrapped().are_type_defs_types_crate() {
                    continue;
                }
                if PATHS_IN_TYPES.contains(&path.as_ref()) {
                    required.push(path.clone());
                    continue;
                }
                let my_crate = component.krate_unwrapped().base();
                let depended_on =
                    graph.neighbors_directed(path, Direction::Incoming).collect::<Vec<_>>();
                let depended_crates = depended_on
                    .iter()
                    .map(|m| self.get(m).krate_unwrapped().for_types())
                    .filter(|c| *c != my_crate)
                    .collect::<IndexSet<_>>();
                if !depended_crates.is_empty() || depended_crates.iter().any(|d| *d == Crate::Types)
                {
                    required.push(path.clone());
                }
            }

            let done = required.is_empty();
            if done {
                break;
            }
            for req in required.drain(..) {
                self.get_mut(&req).krate_unwrapped_mut().set_type_defs_in_types_crate();
            }
        }
    }

    fn maybe_infer_crate_by_what_depends_on_it(
        &self,
        path: &ComponentPath,
        graph: &ComponentGraph,
    ) -> Option<Crate> {
        let depended_on_by = graph
            .neighbors_directed(path, Direction::Incoming)
            .map(|n| self.get(n).krate().map(|krate| krate.base()))
            .collect::<Option<Vec<_>>>()?;

        let first = depended_on_by.first()?;
        if depended_on_by.iter().all(|d| d == first) {
            Some(*first)
        } else {
            None
        }
    }

    fn maybe_infer_crate_by_deps(
        &self,
        path: &ComponentPath,
        graph: &ComponentGraph,
    ) -> Option<Crate> {
        let known_dependents = graph
            .neighbors(path)
            .map(|n| self.get(n).krate().map(|krate| krate.base()))
            .collect::<Option<IndexSet<_>>>()?;

        let first = known_dependents.first()?;
        if known_dependents.iter().all(|d| d == first) {
            Some(*first)
        } else {
            None
        }
    }
}

fn infer_crates_using_deps(
    components: &mut Components,
    infer_test: fn(&Components, &ComponentPath, &ComponentGraph) -> Option<Crate>,
) {
    let mut new_assignments: IndexMap<ComponentPath, Crate> = IndexMap::new();
    loop {
        let graph = components.gen_component_dep_graph();
        for (path, component) in &components.components {
            if component.krate().is_some() {
                continue;
            }
            if let Some(assignment) = infer_test(components, path, &graph) {
                new_assignments.insert(path.clone(), assignment);
            }
        }
        let no_new_assignments = new_assignments.is_empty();

        trace!("Inferred {new_assignments:#?}");
        for (mod_name, krate) in new_assignments.drain(..) {
            components.get_mut(&mod_name).assign_crate(krate);
        }
        if no_new_assignments {
            break;
        }
    }
}
