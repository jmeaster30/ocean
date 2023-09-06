use std::collections::HashMap;
use crate::hydro::analyzer::possiblevalue::PossibleValue;

pub struct AnalysisNode {
    stack: Vec<PossibleValue>
}

pub struct AnalysisGraph {
    nodes: HashMap<u32, AnalysisNode>,
    edges: HashMap<u32, Vec<u32>>,
}
