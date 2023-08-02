use std::collections::HashMap;
use std::fs::File;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use lazy_static::lazy_static;
use petgraph::algo::is_cyclic_directed;
use petgraph::Direction;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::components::{Components, Module, ModuleName};
use crate::graph::ModuleGraph;

// pub const PATHS_IN_TYPES: &[&str] = &[
//     "account",
//     "file",
//     "customer",
//     "payment_source",
//     "invoice",
//     "payment_method",
//     "payment_intent",
//     "product",
// ];

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
    pub fn all() -> &'static [Crate] {
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
        if self == Crate::Types {
            "stripe_types".into()
        } else {
            format!("crates/{}", self.name())
        }
    }

    pub fn name(self) -> String {
        format!("stripe_{}", self.suffix())
    }

    pub fn suffix(self) -> &'static str {
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
fn load_crate_map() -> anyhow::Result<HashMap<Crate, Vec<String>>> {
    let loaded = serde_json::from_reader(File::open("crate_map.json")?)?;
    Ok(loaded)
}

lazy_static! {
    static ref CRATE_MAP: HashMap<Crate, Vec<String>> =
        load_crate_map().expect("Could not load crate map");
}

pub fn validate_crate_map(components: &Components) -> anyhow::Result<()> {
    for members in CRATE_MAP.values() {
        for name in members {
            if !components.modules.contains_key(name.as_str()) {
                bail!("Crate map includes unrecognized {name}. Maybe it is misspelled?");
            }
        }
    }
    Ok(())
}

pub fn maybe_specified_crate(name: &str) -> Option<Crate> {
    for krate in Crate::all() {
        if CRATE_MAP[krate].iter().any(|c| c == name) {
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
        let mut new_assignments: IndexMap<ModuleName, Crate> = IndexMap::new();
        for (mod_name, module) in &self.modules {
            match module {
                Module::Package { .. } => {}
                Module::Component { path, krate } => {
                    if krate.is_some() {
                        continue;
                    }
                    let this_obj = self.get(path);
                    let words = mod_name.split('_');
                    let first_two = words.take(2).collect::<Vec<_>>().join("_");
                    let first_word = mod_name.split_once('_').map(|f| f.0);
                    for (other_mod_name, other_module) in &self.modules {
                        let Some(krate) = other_module.krate() else {
                            continue;
                        };
                        if let Some(obj) = self.maybe_get(other_mod_name.as_ref()) {
                            if this_obj.is_nested_resource_of(obj) {
                                new_assignments.insert(mod_name.clone(), krate.base());
                                break;
                            }
                        };

                        if other_mod_name.as_ref() == first_two {
                            new_assignments.insert(mod_name.clone(), krate.base());
                            break;
                        }
                        if first_word == Some(other_mod_name.as_ref()) {
                            new_assignments.insert(mod_name.clone(), krate.base());
                            break;
                        }
                    }
                }
            }
        }
        debug!("Inferred {new_assignments:#?} based on naming");
        for (mod_name, krate) in new_assignments {
            self.get_module_mut(&mod_name).assign_crate(krate);
        }

        infer_crates_using_deps(self, Self::maybe_infer_crate_by_what_depends_on_it);
        infer_crates_using_deps(self, Self::maybe_infer_crate_by_deps);
        self.assign_uninferrable_crates();
        self.assign_paths_required_to_live_in_types_crate();
        self.validate_crate_assignment()
    }

    fn assign_uninferrable_crates(&mut self) {
        let uninferrable = [
            ("linked_account_options_us_bank_account", Crate::Core),
            ("three_d_secure_details", Crate::Types),
        ];
        for (mod_name, krate) in uninferrable {
            let module = self.modules.get_mut(mod_name).expect("Module not found");
            if module.krate().is_some() {
                panic!(
                    "Module {} mod_name already had crate {:?}. Cannot assign {:?}",
                    mod_name,
                    module.krate().unwrap(),
                    krate
                );
            }
            module.assign_crate(krate);
        }
    }

    fn validate_crate_assignment(&self) -> anyhow::Result<()> {
        let mods_missing_crates = self
            .modules
            .iter()
            .filter(|(_, module)| module.krate().is_none())
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        if !mods_missing_crates.is_empty() {
            let mod_graph = self.gen_module_dep_graph();
            for missing_mod in &mods_missing_crates {
                let depended_on = mod_graph
                    .neighbors_directed(missing_mod, Direction::Incoming)
                    .collect::<Vec<_>>();
                let depended_crates =
                    depended_on.iter().map(|m| self.get_module(m).krate()).collect::<IndexSet<_>>();
                error!("Could not infer crate for {missing_mod}. Depended on by modules {depended_on:?}, crates {depended_crates:?}")
            }
            bail!("Some modules could not have their crate inferred: {:#?}", mods_missing_crates);
        }

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
        Ok(())
    }

    fn assign_paths_required_to_live_in_types_crate(&mut self) {
        let mut required = vec![];
        loop {
            let mod_graph = self.gen_module_dep_graph();
            for (module_name, module) in &self.modules {
                if module.krate_unwrapped().are_type_defs_types_crate() {
                    continue;
                }
                let my_crate = module.krate_unwrapped().base();
                let depended_on = mod_graph
                    .neighbors_directed(module_name, Direction::Incoming)
                    .collect::<Vec<_>>();
                let depended_crates = depended_on
                    .iter()
                    .map(|m| self.get_module(m).krate_unwrapped().for_types())
                    .filter(|c| *c != my_crate)
                    .collect::<IndexSet<_>>();
                if !depended_crates.is_empty() || depended_crates.iter().any(|d| *d == Crate::Types)
                {
                    required.push(module_name.clone());
                }
            }

            debug!("Assigned types for {required:#?} to live in `stripe_types`");
            let done = required.is_empty();
            if done {
                break;
            }
            for req in required.drain(..) {
                self.get_module_mut(&req).krate_unwrapped_mut().set_type_defs_in_types_crate();
            }
        }
    }

    fn maybe_infer_crate_by_what_depends_on_it(
        &self,
        mod_name: &ModuleName,
        mod_graph: &ModuleGraph,
    ) -> Option<Crate> {
        let depended_on_by = mod_graph
            .neighbors_directed(mod_name, Direction::Incoming)
            .map(|n| self.get_module(n).krate().map(|krate| krate.base()))
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
        mod_name: &ModuleName,
        mod_graph: &ModuleGraph,
    ) -> Option<Crate> {
        let known_dependents = mod_graph
            .neighbors(mod_name)
            .map(|n| self.get_module(n).krate().map(|krate| krate.base()))
            .collect::<Option<IndexSet<_>>>()?;

        let first = known_dependents.first()?;
        if known_dependents.iter().all(|d| d == first) {
            Some(*first)
        } else {
            None
        }
    }
}

fn infer_crates_using_deps<F>(components: &mut Components, infer_test: F)
where
    F: Copy + FnOnce(&Components, &ModuleName, &ModuleGraph) -> Option<Crate>,
{
    let mut new_assignments: IndexMap<ModuleName, Crate> = IndexMap::new();
    loop {
        let mod_graph = components.gen_module_dep_graph();
        for (mod_name, module) in &components.modules {
            if module.krate().is_some() {
                continue;
            }
            if let Some(assignment) = infer_test(components, mod_name, &mod_graph) {
                new_assignments.insert(mod_name.clone(), assignment);
            }
        }
        let no_new_assignments = new_assignments.is_empty();

        debug!("Inferred {new_assignments:#?}");
        for (mod_name, krate) in new_assignments.drain(..) {
            components.get_module_mut(&mod_name).assign_crate(krate);
        }
        if no_new_assignments {
            break;
        }
    }
}
