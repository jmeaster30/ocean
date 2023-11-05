use crate::hydro::function::Function;
use crate::hydro::module::Module;
use crate::hydro::value::{Type, Value};

#[test]
fn test_push_return() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .push_value(Value::Unsigned32(420))
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(420))));
}

#[test]
fn test_push_no_return() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .push_value(Value::Unsigned32(420)))
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(None));
}

#[test]
fn test_add() {
  #[rustfmt::skip]
  let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push_value(Value::Unsigned32(420))
        .push_value(Value::Unsigned32(69))
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}

#[test]
fn test_add_with_main_parameter() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .parameter(Type::Unsigned32)
        .push_value(Value::Unsigned32(69))
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), vec![Value::Unsigned32(420)], None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}
