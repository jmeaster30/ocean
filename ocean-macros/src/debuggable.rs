use proc_macro::TokenStream;
use quote::quote;

pub fn debuggable_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl Debuggable for #name {
      fn debug(&self, compilation_unit: &CompilationUnit, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
        let metric_name = stringify!(#name).to_lowercase();
        debug_context.metric_tracker.start(context.get_call_stack(), metric_name.clone());
        let result = self.execute(compilation_unit, context);
        debug_context.metric_tracker.stop(context.get_call_stack(), metric_name);
        return result;
      }
    }
  };
  gen.into()
}