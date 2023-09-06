use crate::hydro::analyzer::possiblevalue::PossibleValue;
use crate::hydro::value::Value;

#[test]
fn test_possiblevalue_union() {
    let left = PossibleValue::range_inc_exc(Value::Unsigned64(1), Value::Unsigned64(4));
    let right = PossibleValue::range_inc_inc(Value::Unsigned64(3), Value::Unsigned64(10));

    let result = PossibleValue::union(left, right);

    assert!(result.contains(Value::Unsigned64(4)));
    assert!(result.contains(Value::Unsigned64(5)));
    assert!(result.contains(Value::Unsigned64(1)));
    assert!(result.contains(Value::Unsigned64(10)));
    assert!(!result.contains(Value::Unsigned64(0)));
    assert!(!result.contains(Value::Unsigned64(12)));
}

#[test]
fn test_possiblevalue_intersect() {
    let left = PossibleValue::range_inc_exc(Value::Unsigned64(1), Value::Unsigned64(6));
    let right = PossibleValue::range_inc_inc(Value::Unsigned64(3), Value::Unsigned64(10));

    let result = PossibleValue::intersect(left, right);

    assert!(result.contains(Value::Unsigned64(4)));
    assert!(result.contains(Value::Unsigned64(3)));
    assert!(!result.contains(Value::Unsigned64(1)));
    assert!(!result.contains(Value::Unsigned64(6)));
    assert!(!result.contains(Value::Unsigned64(0)));
    assert!(!result.contains(Value::Unsigned64(12)));
}

#[test]
fn test_possiblevalue_complement() {
    let value = PossibleValue::range_exc_inc(Value::Unsigned8(17), Value::Unsigned8(64));

    let result = PossibleValue::complement(value);

    assert!(result.contains(Value::Unsigned8(17)));
    assert!(!result.contains(Value::Unsigned8(48)));
    assert!(!result.contains(Value::Unsigned8(64)));
    assert!(result.contains(Value::Unsigned8(128)));
    assert!(result.contains(Value::Unsigned8(0)));
}
