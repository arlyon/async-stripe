use anyhow::bail;
use indexmap::{IndexMap, IndexSet};
use petgraph::algo::is_cyclic_directed;
use petgraph::Direction;
use tracing::{debug, error, trace};

use crate::components::Components;
use crate::crates::{Crate, CRATE_INFO};
use crate::graph::ComponentGraph;
use crate::types::ComponentPath;

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
        debug!("Inferred based on naming: {new_assignments:#?} ");
        for (path, krate) in new_assignments {
            self.get_mut(&path).assign_crate(krate);
        }

        infer_crates_using_deps(self, Self::maybe_infer_crate_by_what_depends_on_it);
        infer_crates_using_deps(self, Self::maybe_infer_crate_by_deps);
        self.ensure_no_missing_crates()?;
        self.assign_paths_required_to_share_types();
        self.validate_crate_assignment()
    }

    /// If we find a component with no assigned crate, return an error with some
    /// additional dependency graph information to help determine why.
    fn ensure_no_missing_crates(&self) -> anyhow::Result<()> {
        let missing_crates = self
            .components
            .iter()
            .filter(|(_, comp)| comp.krate().is_none())
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

        // If we've got missing crates, also look at the dep graph to dump some
        // helpful info
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
        let deps_for_shared_crate = graph.neighbors(Crate::SHARED).collect::<Vec<_>>();
        if !deps_for_shared_crate.is_empty() {
            bail!(
                "Shared types crate should not have dependencies, but has dependencies {:#?}!",
                deps_for_shared_crate
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
        // Paths for components required to live in the `stripe_shared` crate. Adding `event` is
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
