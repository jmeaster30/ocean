use std::collections::HashMap;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
  attributes::*,
  cmd::{CommandArg, Format},
  exec,
  printer::PrinterContext,
};
use crate::hydro::module::Module;


pub struct ModuleDependencyVisualization {
  nodes: Vec<Node>,
  seen_modules: HashMap<String, NodeId>,
  connections: HashMap<String, Vec<NodeId>>
}

impl ModuleDependencyVisualization {
  pub fn create(module: &Module) -> ModuleDependencyVisualization {
    let mut visualization = ModuleDependencyVisualization {
      nodes: Vec::new(),
      seen_modules: HashMap::new(),
      connections: HashMap::new(),
    };

    visualization.generate_internal(module);
    visualization
  }

  pub fn png(&self, output_filename: String)
  {
    exec(
      self.build_graph(),
      &mut PrinterContext::default(),
      vec![
        Format::Png.into(),
        CommandArg::Output(output_filename)
      ],
    ).unwrap();
  }

  pub fn svg(&self, output_filename: String)
  {
    exec(
      self.build_graph(),
      &mut PrinterContext::default(),
      vec![
        Format::Svg.into(),
        CommandArg::Output(output_filename)
      ],
    ).unwrap();
  }

  fn build_graph(&self) -> Graph {
    let mut graph = graph!(strict di id!("mod_viz_graph"));

    for node in &self.nodes {
      graph.add_stmt(Stmt::Node(node.clone()));
    }

    for (node_name, connected_nodes) in &self.connections {
      match self.seen_modules.get(node_name.as_str()) {
        Some(node_id) => for end_node_id in connected_nodes {
          graph.add_stmt(Stmt::Edge(edge!(node_id.clone() => end_node_id.clone())))
        }
        None => panic!("shouldn't have happened :(")
      }
    }

    graph
  }

  fn generate_internal(&mut self, module: &Module)
  {
    if self.seen_modules.contains_key(module.name.clone().as_str()) {
      return
    }
    let module_node = node!(module.name.clone().as_str());
    let module_id = module_node.id.clone();
    self.nodes.push(module_node);
    self.seen_modules.insert(module.name.clone(), module_id.clone());
    self.connections.insert(module.name.clone(), Vec::new());
    for (dep_mod_name, dep_mod) in &module.modules {
      self.generate_internal(dep_mod);
      match self.connections.get_mut(module.name.clone().as_str()) {
        Some(connect) => connect.push(self.seen_modules.get(dep_mod_name.as_str()).unwrap().clone()),
        None => panic!("should've happened :(")
      }
    }
  }
}

pub fn test() {
  let mut g = graph!(id!("id");
         node!("nod"),
         subgraph!("sb";
             edge!(node_id!("a") => subgraph!(;
                node!("n";
                NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
            ))
        ),
        edge!(node_id!("a1") => node_id!(esc "a2"))
    );

  let graph_svg = exec(
    g,
    &mut PrinterContext::default(),
    vec![
      Format::Svg.into(),
      CommandArg::Output("dot.svg".to_string())
    ],
  ).unwrap();
}
