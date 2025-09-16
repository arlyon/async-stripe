use indexmap::IndexSet;
use petgraph::prelude::DiGraphMap;

use crate::components::Components;
use crate::crates::{Crate, ALL_CRATES};
use crate::types::ComponentPath;

pub type ComponentGraph<'a> = DiGraphMap<&'a ComponentPath, ()>;

impl Components {
    /// Generate a dependency graph with an edge from A to B implying that A depends on B
    pub fn gen_component_dep_graph(&self) -> ComponentGraph<'_> {
        let mut graph = DiGraphMap::new();
        for path in self.components.keys() {
            graph.add_node(path);
        }

        for path in self.components.keys() {
            let deps = self.deps_for_component(path);
            for dep in deps {
                // Don't clutter with self edges since they aren't particularly meaningful
                // in this context
                if dep != path {
                    graph.add_edge(path, dep, ());
                }
            }
        }
        graph
    }

    pub fn gen_crate_dep_graph(&self) -> DiGraphMap<Crate, ()> {
        let mut graph = DiGraphMap::new();

        for krate in &*ALL_CRATES {
            graph.add_node(*krate);
            if *krate != Crate::SHARED {
                // Everybody depends on `async-stripe-shared` because of reexports
                graph.add_edge(*krate, Crate::SHARED, ());
            }
        }

        for krate in &*ALL_CRATES {
            let members = self.get_crate_members(*krate);
            let mut deps = IndexSet::new();
            for member in members {
                let comp_deps = self.deps_for_component(member);
                deps.extend(comp_deps);
            }
            for dep in deps {
                let dependent_crate = self.get(dep).krate_unwrapped().for_types();
                // Don't clutter with self edges since they aren't particularly meaningful
                // in this context
                if dependent_crate != *krate {
                    graph.add_edge(*krate, dependent_crate, ());
                }
            }
        }
        graph
    }
}
