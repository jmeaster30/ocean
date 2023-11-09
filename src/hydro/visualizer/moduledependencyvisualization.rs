use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
  cmd::{CommandArg, Format},
  exec,
  printer::PrinterContext,
};
use std::collections::HashMap;
use crate::hydro::compilationunit::CompilationUnit;

pub struct ModuleDependencyVisualization {
  nodes: Vec<Node>,
  seen_modules: HashMap<String, NodeId>,
  connections: HashMap<String, Vec<NodeId>>,
}

impl ModuleDependencyVisualization {
  pub fn create(compilation_unit: &CompilationUnit, module: &String) -> ModuleDependencyVisualization {
    let mut visualization = ModuleDependencyVisualization { nodes: Vec::new(), seen_modules: HashMap::new(), connections: HashMap::new() };

    visualization.generate_internal(compilation_unit, module);
    visualization
  }

  pub fn png(&self, output_filename: String) {
    exec(self.build_graph(), &mut PrinterContext::default(), vec![Format::Png.into(), CommandArg::Output(output_filename)]).unwrap();
  }

  pub fn svg(&self, output_filename: String) {
    exec(self.build_graph(), &mut PrinterContext::default(), vec![Format::Svg.into(), CommandArg::Output(output_filename)]).unwrap();
  }

  fn build_graph(&self) -> Graph {
    let mut graph = graph!(strict di id!("mod_viz_graph"));

    for node in &self.nodes {
      graph.add_stmt(Stmt::Node(node.clone()));
    }

    for (node_name, connected_nodes) in &self.connections {
      match self.seen_modules.get(node_name.as_str()) {
        Some(node_id) => {
          for end_node_id in connected_nodes {
            graph.add_stmt(Stmt::Edge(edge!(node_id.clone() => end_node_id.clone())))
          }
        }
        None => panic!("shouldn't have happened :("),
      }
    }

    graph
  }

  fn generate_internal(&mut self, compilation_unit: &CompilationUnit, module_name: &String) {
    if self.seen_modules.contains_key(module_name) {
      return;
    }
    let module_node = node!(module_name.as_str());
    let module_id = module_node.id.clone();
    self.nodes.push(module_node);
    self.seen_modules.insert(module_name.clone(), module_id.clone());
    self.connections.insert(module_name.clone(), Vec::new());

    let this_module = compilation_unit.get_module(module_name).unwrap();
    for dependency in &this_module.modules {
      self.generate_internal(compilation_unit, dependency);
      match self.connections.get_mut(module_name.as_str()) {
        Some(connect) => connect.push(self.seen_modules.get(dependency.as_str()).unwrap().clone()),
        None => panic!("should've happened :("),
      }
    }
  }
}
