use crate::hydro::analyzer::analysiscontext::AnalysisContext;

pub trait Analyzable {
  fn analyze(&self, context: &mut AnalysisContext);
}