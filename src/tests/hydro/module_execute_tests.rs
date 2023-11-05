use crate::hydro::function::Function;
use crate::hydro::module::Module;
use crate::hydro::value::{Type, Value};

#[test]
fn test_push_return() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(420))
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(420))));
}

#[test]
fn test_pop() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(420))
        .push(Value::Unsigned32(69))
        .pop()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(420))));
}

#[test]
fn test_duplicate() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(100))
        .duplicate(0)
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(200))));
}

#[test]
fn test_duplicate_with_offset() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(100))
        .push(Value::Unsigned32(50))
        .duplicate(1)
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(150))));
}

#[test]
fn test_duplicate_with_offset_2() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(100))
        .push(Value::Unsigned32(50))
        .duplicate(1)
        .add()
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(200))));
}

#[test]
fn test_subtract() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Signed32(100))
        .push(Value::Signed32(50))
        .subtract()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Signed32(50))));
}

#[test]
fn test_swap_subtract() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Signed32(100))
        .push(Value::Signed32(50))
        .swap()
        .subtract()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Signed32(-50))));
}

#[test]
fn test_rotate_3() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(3))
        .push(Value::Unsigned32(2))
        .push(Value::Unsigned32(1))
        .rotate(3)
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(4))));
}

#[test]
fn test_rotate_minus3() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(3))
        .push(Value::Unsigned32(2))
        .push(Value::Unsigned32(1))
        .rotate(-3)
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(5))));
}

#[test]
fn test_push_no_return() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(420)))
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
        .push(Value::Unsigned32(420))
        .push(Value::Unsigned32(69))
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}

#[test]
fn test_multiply() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(420))
        .push(Value::Unsigned32(69))
        .multiply()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(420 * 69))));
}

#[test]
fn test_multiply_negative() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Signed32(-10))
        .push(Value::Signed32(13))
        .multiply()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Signed32(-130))));
}

#[test]
fn test_divide() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Signed32(-100))
        .push(Value::Signed32(10))
        .divide()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Signed32(-10))));
}

#[test]
fn test_divide_by_zero() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Signed32(15))
        .push(Value::Signed32(0))
        .divide()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  match return_value {
    Ok(_) => assert!(false, "Return value had a value but it shouldn't have :("),
    Err(exception) => assert_eq!(exception.message, "Attempt to divide by zero :("),
  }
}

#[test]
fn test_modulo() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(15))
        .push(Value::Signed32(4))
        .modulo()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Signed64(3))));
}

#[test]
fn test_leftshift() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(1))
        .push(Value::Unsigned8(5))
        .leftshift()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(32))));
}

#[test]
fn test_rightshift() {
  #[rustfmt::skip]
    let module =  {
    Module::build("main")
      .function(Function::build("main")
        .push(Value::Unsigned32(64))
        .push(Value::Unsigned8(2))
        .rightshift()
        .ret())
  };

  let return_value = module.execute("main".to_string(), Vec::new(), None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(16))));
}

#[test]
fn test_add_with_main_parameter() {
  #[rustfmt::skip]
  let module = {
    Module::build("main")
      .function(Function::build("main")
        .parameter(Type::Unsigned32)
        .push(Value::Unsigned32(69))
        .add()
        .ret())
  };

  let return_value = module.execute("main".to_string(), vec![Value::Unsigned32(420)], None);

  assert_eq!(return_value, Ok(Some(Value::Unsigned32(489))));
}
