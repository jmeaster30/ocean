use crate::hydro::function::Function;
use crate::hydro::module::Module;
use crate::hydro::value::Value;

#[test]
fn test_push_return() {
  let module = Module::build("main")
    .function(Function::build("main")
      .push_value(Value::Unsigned32(420))
      .return_());

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(420))));
}

#[test]
fn test_push_no_return() {
  let module = Module::build("main")
    .function(Function::build("main")
      .push_value(Value::Unsigned32(420)));

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(None));
}

#[test]
fn test_add() {
  let module = Module::build("main")
    .function(Function::build("main")
      .push_value(Value::Unsigned32(420))
      .push_value(Value::Unsigned32(69))
      .add()
      .return_());

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}

#[test]
fn test_add_with_variable() {
  let module = Module::build("main")
    .function(Function::build("main")
      .parameter("coolNumber")
      .var_ref("coolNumber")
      .load()
      .push_value(Value::Unsigned32(69))
      .add()
      .return_());

  let return_value = module.execute("main".to_string(), vec![("coolNumber".to_string(), Value::Unsigned32(420))], None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}