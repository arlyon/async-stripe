use std::collections::{HashMap, HashSet};

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::prelude::Bfs;
use petgraph::Graph;

use crate::codegen::CodeGen;
use crate::components::{Components, Module};
use crate::types::ComponentPath;

impl CodeGen {
    pub fn get_graphviz_dep_graph(&self) -> String {
        let graph = self.components.gen_dep_graph();
        format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))
    }

    pub fn _get_component_dependencies(&self, component: &ComponentPath) -> Vec<String> {
        let dep_graph = self.components.gen_dep_graph();
        let index = dep_graph.node_indices().find(|i| dep_graph[*i] == component.as_ref()).unwrap();
        let mut bfs = Bfs::new(&dep_graph, index);
        let mut dependencies = vec![];
        while let Some(nx) = bfs.next(&dep_graph) {
            dependencies.push(dep_graph[nx].clone());
        }
        dependencies
    }
}

impl Components {
    fn get_deps_from_path(&self, path: &ComponentPath) -> HashSet<String> {
        let component = self.get(path);
        let base_type = component.rust_obj();

        let mut dep_paths = HashSet::new();
        for dep in base_type.data.schema_deps() {
            dep_paths.insert(self.containing_module(dep));
        }
        dep_paths
    }

    fn add_deps_to_graph(
        &self,
        graph: &mut Graph<String, ()>,
        node_map: &HashMap<String, NodeIndex>,
        src_mod: &str,
        path: &ComponentPath,
    ) {
        let deps = self.get_deps_from_path(path);
        let src_node_id = &node_map[src_mod];
        for dep in deps {
            // Don't clutter with self edges since they aren't particularly meaningful
            // in this context
            if dep != src_mod {
                let dest_node_id = &node_map[&dep];
                graph.add_edge(*src_node_id, *dest_node_id, ());
            }
        }
    }

    pub fn gen_dep_graph(&self) -> Graph<String, ()> {
        let mut graph: Graph<String, ()> = Graph::new();
        let mut node_map = HashMap::new();
        for mod_name in self.modules.keys() {
            let node_ind = graph.add_node(mod_name.to_string());
            node_map.insert(mod_name.clone(), node_ind);
        }
        for (mod_name, module) in &self.modules {
            match module {
                Module::Package(_, paths) => {
                    for path in paths {
                        self.add_deps_to_graph(&mut graph, &node_map, mod_name, path);
                    }
                }
                Module::Component(path) => {
                    self.add_deps_to_graph(&mut graph, &node_map, mod_name, path);
                }
            }
        }
        graph
    }
}
