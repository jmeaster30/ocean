use crate::ocean::frontend::ast::*;

pub fn parse_annotations(ast: &mut Program) {
  for node in &mut ast.statements {
    parse_annotation_statement_node(node);
  }
}

fn parse_annotation_statement_node(statement_node: &mut StatementNode) {
  for data in &mut statement_node.data {
    match data {
      StatementNodeData::Annotation(annotation) => annotation.annotation_ast = Some(annotation_parser(&annotation.token.lexeme)),
      _ => {}
    }
  }
  match &mut statement_node.statement {
    Some(statement) => parse_annotation_statement(statement),
    None => {}
  }
}

fn parse_annotation_statement(statement: &mut Statement) {
  // TODO: need to move annotations into unions and packs so
}

fn annotation_parser(annotation_content: &String) -> AnnotationNode {
  let trimmed_content = annotation_content.trim_matches(&['@', '\n', ' ', '\t'] as &[_]);
  let (annotation_name, annotation_body) = match trimmed_content.split_once(char::is_whitespace) {
    Some(annotation) => annotation,
    None => (trimmed_content, ""),
  };

  match annotation_name.to_lowercase().as_str() {
    "hydro" => println!("found some hydro code"),
    "annotation" => println!("found an annotation!"),
    "operator" => println!("found an operator"),
    _ => println!("oooo a custom annotation!"),
  }

  AnnotationNode::None
}
