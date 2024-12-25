use crate::ocean::frontend::ast::node::{Annotation, Expression, ExpressionNode};
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;

pub enum AnnotationType {
  Operator,
  Custom(String),
}

pub enum OperatorOrder {
  Prefix,
  Postfix,
  Infix,
}

impl Annotation {
  pub fn get_argument_expression(&self, name: &str) -> Result<ExpressionNode, Error> {
    match self.annotation_arguments.iter().find(|x| x.name.lexeme.to_lowercase() == name) {
      Some(argument) => Ok(argument.value.clone()),
      None => Err(Error::new(Severity::Error, self.get_span(), format!("Could not find argument '{}'", name)))
    }
  }

  pub fn get_annotation_type(&self) -> AnnotationType {
    match self.token.lexeme.to_lowercase().as_str() {
      "@operator" => AnnotationType::Operator,
      _ => AnnotationType::Custom(self.token.lexeme.strip_prefix("@").unwrap().to_string()),
    }
  }

  pub fn get_operator_order(&self) -> Result<OperatorOrder, Error> {
    let expression_node = self.get_argument_expression("order")?;
    match expression_node.clone().parsed_expression {
      Some(expr) => {
        let operator_order = match expr {
          Expression::BinaryOperation(binary_operation) => {
            if binary_operation.operator.lexeme != "." {
              return Err(Error::new(Severity::Error, expression_node.get_span(), "Unexpected expression for operator order".to_string()))
            }

            match (binary_operation.left_expression.as_ref(), binary_operation.right_expression.as_ref()) {
              (Expression::Variable(operator_order_enum), Expression::Variable(operator_order)) if operator_order_enum.identifier.lexeme.to_lowercase() == "operatororder" => match operator_order.identifier.lexeme.to_lowercase().as_str() {
                "infix" => OperatorOrder::Infix,
                "prefix" => OperatorOrder::Prefix,
                "postfix" => OperatorOrder::Postfix,
                _ => return Err(Error::new(Severity::Error, expression_node.get_span(), format!("Unexpected operator order. Must be one of 'infix', 'prefix', or 'postfix'."))),
              }
              (Expression::Variable(operator_order_enum), Expression::Variable(operator_order)) if operator_order_enum.identifier.lexeme.to_lowercase() != "operatororder" => return Err(Error::new(Severity::Error, expression_node.get_span(), "Expected enum 'OperatorOrder'".to_string())),
              _ => return Err(Error::new(Severity::Error, expression_node.get_span(), "Unexpected expression for operator order".to_string()))
            }
          }
          Expression::Variable(variable) => match variable.identifier.lexeme.to_lowercase().as_str() {
            "infix" => OperatorOrder::Infix,
            "prefix" => OperatorOrder::Prefix,
            "postfix" => OperatorOrder::Postfix,
            _ => return Err(Error::new(Severity::Error, expression_node.get_span(), format!("Unexpected operator order. Must be one of 'infix', 'prefix', or 'postfix'."))),
          }
          _ => return Err(Error::new(Severity::Error, expression_node.get_span(), "Unexpected expression for operator order".to_string()))
        };

        Ok(operator_order)
      }
      None => panic!("uh oh"),
    }
  }

  pub fn get_operator_symbol(&self) -> Result<String, Error> {
    let expression_node = self.get_argument_expression("symbol")?;
    match expression_node.clone().parsed_expression {
      Some(expr) => match expr {
        Expression::String(string_literal) => Ok(string_literal.token.lexeme.strip_prefix(&['\'', '"']).unwrap().to_string().strip_suffix(&['\'', '"']).unwrap().to_string()),
        _ => Err(Error::new(Severity::Error, expression_node.get_span(), "Unexpected expression for operator symbol".to_string()))
      }
      None => panic!("uh oh"),
    }
  }

  pub fn get_operator_infix_precedence(&self) -> Result<(Option<usize>, Option<usize>), Error> {
    let expression_node = match self.get_argument_expression("precedence") {
      Ok(expr) => expr,
      Err(_) => return Ok((None, None))
    };
    match expression_node.clone().parsed_expression {
      Some(expr) => match expr {
        Expression::Tuple(precedence_tuple) => {
          if precedence_tuple.tuple_members.len() != 2 {
            return Err(Error::new(Severity::Error, expression_node.get_span(), "Expected tuple to have 2 entries".to_string()));
          }

          let left = match precedence_tuple.tuple_members.get(0) {
            Some(tuple_member) => Annotation::get_usize(tuple_member.value.clone(), tuple_member.get_span())?,
            None => panic!()
          };

          let right = match precedence_tuple.tuple_members.get(1) {
            Some(tuple_member) => Annotation::get_usize(tuple_member.value.clone(), tuple_member.get_span())?,
            None => panic!()
          };

          Ok((left, right))
        }
        _ => Err(Error::new(Severity::Error, expression_node.get_span(), "Unexpected expression for infix operator precedence".to_string())),
      }
      None => panic!("oh no"),
    }
  }

  fn get_usize(expression: Expression, span: (usize, usize)) -> Result<Option<usize>, Error> {
    match expression {
      Expression::Number(number) => match number.token.lexeme.parse::<usize>() {
        Ok(x) => Ok(Some(x)),
        Err(_) => Err(Error::new(Severity::Error, span, "Expected a positive integer".to_string())),
      }
      _ => Err(Error::new(Severity::Error, span, "Unexpected expression for operator precedence".to_string())),
    }
  }

  pub fn get_operator_prefix_postfix_precedence(&self) -> Result<Option<usize>, Error> {
    let expression_node = match self.get_argument_expression("precedence") {
      Ok(expr) => expr,
      Err(_) => return Ok(None)
    };
    match expression_node.clone().parsed_expression {
      Some(expr) => Annotation::get_usize(expr, expression_node.get_span()),
      _ => panic!("uh oh")
    }
  }
}