use std::collections::HashMap;
use std::fmt;

use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use petgraph::Direction;
use petgraph::algo::is_cyclic_directed;
use tracing::{debug, trace, warn};

use crate::components::Components;
use crate::crates::{CRATE_INFO, Crate};
use crate::graph::ComponentGraph;
use crate::types::ComponentPath;

/// A warning emitted when a component's crate had to be auto-resolved
/// because normal inference couldn't determine a unique crate.
#[derive(Debug, Clone)]
pub struct CrateInferenceWarning {
    pub component: ComponentPath,
    pub chosen_crate: Crate,
    pub depended_on_by: Vec<ComponentPath>,
    pub candidate_crates: Vec<Crate>,
}

impl fmt::Display for CrateInferenceWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Auto-assigned `{}` to crate `{}` (depended on by: {}, candidate crates: {})",
            self.component,
            self.chosen_crate.base_name(),
            self.depended_on_by.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "),
            self.candidate_crates
                .iter()
                .map(|c| c.base_name().to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

/// Do some easy work to sanity check `gen_crates.toml` does not have any spurious or
/// misspelled entries.
pub fn validate_crate_info(components: &Components) -> anyhow::Result<()> {
    for krate in &CRATE_INFO.crates {
        for name in &krate.paths {
            if !components.components.contains_key(name.as_str()) {
                bail!("Crate info includes unrecognized {name}. Maybe it is misspelled?");
            }
        }
    }
    Ok(())
}

impl Components {
    #[tracing::instrument(skip_all)]
    pub fn infer_all_crate_assignments(&mut self) -> anyhow::Result<Vec<CrateInferenceWarning>> {
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
        debug!("Inferred based on naming: {new_assignments:#?} ");
        for (path, krate) in new_assignments {
            self.get_mut(&path).assign_crate(krate);
        }

        infer_crates_using_deps(self, Self::maybe_infer_crate_by_what_depends_on_it);
        infer_crates_using_deps(self, Self::maybe_infer_crate_by_deps);
        let warnings = self.resolve_missing_crates();
        self.assign_paths_required_to_share_types();
        self.validate_crate_assignment()?;
        Ok(warnings)
    }

    /// For components with no assigned crate, auto-resolve by picking the most
    /// common crate among dependents. Returns warnings for each auto-resolved component.
    #[tracing::instrument(skip_all)]
    fn resolve_missing_crates(&mut self) -> Vec<CrateInferenceWarning> {
        let components_without_crates: Vec<ComponentPath> = self
            .components
            .iter()
            .filter(|(_, comp)| comp.krate().is_none())
            .map(|(p, _)| p.clone())
            .collect();

        if components_without_crates.is_empty() {
            return Vec::new();
        }

        let graph = self.gen_component_dep_graph();
        let mut warnings = Vec::new();

        for missing in &components_without_crates {
            let depended_on: Vec<ComponentPath> =
                graph.neighbors_directed(missing, Direction::Incoming).cloned().collect();
            let candidate_crates: Vec<Crate> =
                depended_on.iter().filter_map(|m| self.get(m).krate().map(|k| k.base())).collect();

            // Pick the most common crate, breaking ties alphabetically
            let chosen = if candidate_crates.is_empty() {
                Crate::SHARED
            } else {
                let mut counts: HashMap<Crate, usize> = HashMap::new();
                for c in &candidate_crates {
                    *counts.entry(*c).or_default() += 1;
                }
                let max_count = *counts.values().max().unwrap();
                let mut best: Vec<Crate> =
                    counts.into_iter().filter(|(_, c)| *c == max_count).map(|(k, _)| k).collect();
                best.sort_by_key(|c| c.base_name());
                best[0]
            };

            let unique_crates: Vec<Crate> =
                candidate_crates.iter().copied().collect::<IndexSet<_>>().into_iter().collect();

            warn!(
                "Auto-assigning component {missing} to crate {} (depended on by {depended_on:?}, \
                 candidate crates: {unique_crates:?}). Consider adding a path entry in gen_crates.toml.",
                chosen.base_name()
            );

            warnings.push(CrateInferenceWarning {
                component: missing.clone(),
                chosen_crate: chosen,
                depended_on_by: depended_on,
                candidate_crates: unique_crates,
            });
        }

        for warning in &warnings {
            self.get_mut(&warning.component).assign_crate(warning.chosen_crate);
        }

        warnings
    }

    fn validate_crate_assignment(&self) -> anyhow::Result<()> {
        let graph = self.gen_crate_dep_graph();
        let deps_for_shared_crate = graph.neighbors(Crate::SHARED).collect::<Vec<_>>();
        if !deps_for_shared_crate.is_empty() {
            bail!(
                "Shared types crate should not have dependencies, but has dependencies {deps_for_shared_crate:#?}!",
            );
        }
        if is_cyclic_directed(&graph) {
            bail!("Crate dependency graph contains a cycle!");
        }
        let requests_in_types = self
            .components
            .iter()
            .filter(|(_, comp)| {
                comp.krate_unwrapped().base() == Crate::SHARED && !comp.requests.is_empty()
            })
            .map(|(path, _)| path)
            .collect::<Vec<_>>();
        if !requests_in_types.is_empty() {
            bail!("Components have requests, not allowed in types crate: {requests_in_types:#?}");
        }
        Ok(())
    }

    fn assign_paths_required_to_share_types(&mut self) {
        // Paths for components required to live in the `async-stripe-shared` crate. Adding `event` is
        // used for webhooks
        const PATHS_IN_TYPES: &[&str] = &["event"];

        loop {
            let mut required = vec![];
            let graph = self.gen_component_dep_graph();
            for (path, component) in &self.components {
                if component.krate_unwrapped().are_type_defs_shared() {
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
                if !depended_crates.is_empty()
                    || depended_crates.iter().any(|d| *d == Crate::SHARED)
                {
                    required.push(path.clone());
                }
            }

            let done = required.is_empty();
            if done {
                break;
            }
            for req in required {
                self.get_mut(&req).krate_unwrapped_mut().set_share_type_defs();
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
        if depended_on_by.iter().all(|d| d == first) { Some(*first) } else { None }
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
        if known_dependents.iter().all(|d| d == first) { Some(*first) } else { None }
    }
}

fn infer_crates_using_deps(
    components: &mut Components,
    infer_test: fn(&Components, &ComponentPath, &ComponentGraph) -> Option<Crate>,
) {
    loop {
        let mut new_assignments: IndexMap<ComponentPath, Crate> = IndexMap::new();
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
        for (mod_name, krate) in new_assignments {
            components.get_mut(&mod_name).assign_crate(krate);
        }
        if no_new_assignments {
            break;
        }
    }
}
