use indexmap::IndexSet;
use petgraph::prelude::DiGraphMap;

use crate::components::{Components, ModuleName};
use crate::crate_inference::Crate;

pub type ModuleGraph<'a> = DiGraphMap<&'a ModuleName, ()>;

impl Components {
    /// Generate a dependency graph with an edge from A to B implying that A depends on B
    pub fn gen_module_dep_graph(&self) -> ModuleGraph {
        let mut graph = DiGraphMap::new();
        for mod_name in self.modules.keys() {
            graph.add_node(mod_name);
        }

        for (mod_name, module) in &self.modules {
            let deps = self.deps_for_module(module);
            for dep in deps {
                let dependent_mod = self.containing_module(dep);

                // Don't clutter with self edges since they aren't particularly meaningful
                // in this context
                if dependent_mod != mod_name {
                    graph.add_edge(mod_name, dependent_mod, ());
                }
            }
        }
        graph
    }

    pub fn gen_crate_dep_graph(&self) -> DiGraphMap<Crate, ()> {
        let mut graph = DiGraphMap::new();

        for krate in Crate::all().iter() {
            graph.add_node(*krate);
            if *krate != Crate::Types {
                // Everybody depends on `stripe_types` because of def_id!, ext types, etc.
                graph.add_edge(*krate, Crate::Types, ());
            }
        }
        for krate in Crate::all().iter() {
            let members = self.get_crate_members(*krate);
            let mut deps = IndexSet::new();
            for member in members {
                let module = self.modules.get(member).unwrap();
                let module_deps = self.deps_for_module(module);
                deps.extend(module_deps);
            }
            for dep in deps {
                let dependent_crate = self.containing_crate(dep).for_types();
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
