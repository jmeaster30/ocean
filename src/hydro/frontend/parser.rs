use crate::hydro::frontend::token::{Token, TokenType};
use crate::hydro::function::Function;
use crate::hydro::instruction::{Add, Allocate, And, ArrayIndex, BitwiseAnd, BitwiseNot, BitwiseOr, BitwiseXor, Branch, Call, Divide, Duplicate, Equal, GreaterThan, GreaterThanEqual, Instruction, Jump, LayoutIndex, LeftShift, LessThan, LessThanEqual, Load, Modulo, Multiply, Not, NotEqual, Or, PopValue, PushValue, Return, RightShift, Store, Subtract, Swap, Xor};
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::module::Module;
use crate::hydro::value::{
  Array, FunctionPointer, LayoutIndexRef, Reference, Type, Value, VariableRef,
};
use crate::util::tokentrait::TokenTrait;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct Parser {
  file_contents: Vec<char>,
  current_token: Option<Token>,
  current_index: usize,
  current_line: usize,
  current_column: usize,
}

impl Parser {
  pub fn new(path: &std::path::Path) -> io::Result<Self> {
    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    Ok(Self {
      file_contents: file_contents.chars().collect::<Vec<char>>(),
      current_token: None,
      current_index: 0,
      current_line: 1,
      current_column: 1,
    })
  }

  pub fn parse(&mut self) -> Vec<Module> {
    let mut modules = Vec::new();
    while self.token().is_some() {
      let module = self.parse_module();
      modules.push(module);
    }
    modules
  }

  fn parse_module(&mut self) -> Module {
    let _ = self.expect_token_type(TokenType::Module);
    self.consume();

    let identifier_token = self.expect_one_of(vec![TokenType::Identifier, TokenType::Main]);
    self.consume();

    let mut module = Module::build(identifier_token.lexeme.as_str());

    loop {
      let Some(current_token) = self.token() else {
        break
      };

      match current_token.token_type {
        TokenType::Using => {
          let module_name = self.parse_using();
          module = module.import_unresolved(module_name);
        }
        TokenType::Layout => {
          let layout = self.parse_layout();
          module = module.layout(layout);
        }
        TokenType::Main | TokenType::Function => {
          let func = self.parse_function();
          module = module.function(func);
        }
        TokenType::Module => {
          break;
        }
        _ => panic!(
          "Unexpected token in module statement. Expected 'using', 'layout', 'main', or 'function'"
        ),
      }
    }

    module
  }

  fn parse_using(&mut self) -> String {
    let _ = self.expect_token_type(TokenType::Using);
    self.consume();
    let identifier_token = self.expect_token_type(TokenType::Identifier);
    self.consume();
    identifier_token.lexeme
  }

  fn parse_function(&mut self) -> Function {
    let function_token = self.expect_one_of(vec![TokenType::Function, TokenType::Main]);
    match function_token.token_type {
      TokenType::Function => self.consume(),
      TokenType::Main => { /* DONT CONSUME HERE */ }
      _ => {}
    }

    let identifier_token = self.expect_one_of(vec![TokenType::Identifier, TokenType::Main]);
    self.consume();

    let mut function = Function::build(identifier_token.lexeme.as_str());
    // parse params
    loop {
      let id_token = self.expect_one_of(vec![TokenType::Identifier, TokenType::Body]);
      match id_token.token_type {
        TokenType::Identifier => self.consume(),
        TokenType::Body => break,
        _ => {}
      }
      function = function.parameter(id_token.lexeme.as_str());
    }

    let _ = self.expect_token_type(TokenType::Body);
    self.consume();

    // parse insts
    loop {
      let Some(inst_token) = self.token() else {
        break
      };
      match inst_token.token_type {
        TokenType::Alloc
        | TokenType::Push
        | TokenType::Pop
        | TokenType::Duplicate
        | TokenType::Swap
        | TokenType::Add
        | TokenType::Subtract
        | TokenType::Multiply
        | TokenType::Divide
        | TokenType::Modulo
        | TokenType::LeftShift
        | TokenType::RightShift
        | TokenType::BitwiseAnd
        | TokenType::BitwiseOr
        | TokenType::BitwiseXor
        | TokenType::BitwiseNot
        | TokenType::And
        | TokenType::Or
        | TokenType::Xor
        | TokenType::Not
        | TokenType::Equal
        | TokenType::NotEqual
        | TokenType::LessThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThan
        | TokenType::GreaterThanEqual
        | TokenType::Jump
        | TokenType::Branch
        | TokenType::Call
        | TokenType::Return
        | TokenType::Load
        | TokenType::Store
        | TokenType::Index => {
          let instruction = self.parse_instruction();
          function = function.inst(instruction);
        }
        TokenType::Module
        | TokenType::Function
        | TokenType::Layout
        | TokenType::Using
        | TokenType::Main => break,
        token_type => panic!(
          "Expected to have an instruction here but read a {:?} :(",
          token_type
        ),
      }
    }

    function
  }

  pub fn parse_type(&mut self) -> Type {
    // <type> -> type
    // <id> <id> -> layout
    // <number> <Type>
    let start_token = self.expect_one_of(vec![
      TokenType::Type,
      TokenType::Identifier,
      TokenType::This,
      TokenType::Number,
    ]);
    self.consume();

    match start_token.token_type {
      TokenType::Type => match start_token.lexeme.as_str() {
        "bool" => Type::Boolean,
        "u8" => Type::Unsigned8,
        "u16" => Type::Unsigned16,
        "u32" => Type::Unsigned32,
        "u64" => Type::Unsigned64,
        "u128" => Type::Unsigned128,
        "s8" => Type::Signed8,
        "s16" => Type::Signed16,
        "s32" => Type::Signed32,
        "s64" => Type::Signed64,
        "s128" => Type::Signed128,
        _ => panic!("Unexpected type string"),
      },
      TokenType::Identifier | TokenType::This => {
        let layout_token = self.expect_token_type(TokenType::Identifier);
        self.consume();
        Type::Layout(start_token.lexeme, layout_token.lexeme, None)
      }
      TokenType::Number => {
        let length = start_token.lexeme.parse::<u64>().unwrap();
        let subtype = self.parse_type();

        Type::Array(length, Box::new(subtype))
      }
      _ => panic!("Unexpected token type"),
    }
  }

  fn parse_instruction(&mut self) -> Instruction {
    let inst_token = self.expect_token();
    self.consume();

    match inst_token.token_type {
      TokenType::Alloc => {
        let allocated_type = self.parse_type();
        Instruction::Allocate(Allocate { allocated_type })
      }
      TokenType::Push => {
        let type_token = self.expect_token();
        match type_token.token_type {
          TokenType::Type => self.consume(),
          TokenType::VariableRef => { /*DONT CONSUME*/ }
          TokenType::IndexRef => { /*DONT CONSUME*/ }
          TokenType::FunctionPointer => { /*DONT CONSUME*/ }
          TokenType::Identifier => { /*DONT CONSUME*/ }
          _ => {}
        }

        let value_token = self.expect_token();
        Instruction::PushValue(PushValue {
          value: match value_token.token_type {
            TokenType::Number | TokenType::String => {
              self.consume();
              match Parser::create_value_from_type_string(
                type_token.lexeme,
                value_token.lexeme.clone(),
              ) {
                Ok(value) => value,
                Err(message) => panic!("{}", message),
              }
            }
            TokenType::True => {
              self.consume();
              Value::Boolean(true)
            }
            TokenType::False => {
              self.consume();
              Value::Boolean(false)
            }
            TokenType::VariableRef | TokenType::IndexRef => {
              Value::Reference(self.parse_reference())
            }
            TokenType::FunctionPointer => {
              self.consume();

              let module_token = self.expect_one_of(vec![TokenType::Identifier, TokenType::This, TokenType::Main]);
              self.consume();

              let function_token = self.expect_token_type(TokenType::Identifier);
              self.consume();

              Value::FunctionPointer(FunctionPointer {
                module: match module_token.token_type {
                  TokenType::Identifier | TokenType::Main => Some(module_token.lexeme),
                  TokenType::This => None,
                  _ => panic!("Shouldn't be hit"),
                },
                function: function_token.lexeme,
              })
            }
            _ => panic!("Expected to have a value token here :( {:?}", value_token),
          },
        })
      }
      TokenType::Pop => Instruction::PopValue(PopValue {}),
      TokenType::Duplicate => Instruction::Duplicate(Duplicate {}),
      TokenType::Swap => Instruction::Swap(Swap {}),
      TokenType::Add => Instruction::Add(Add {}),
      TokenType::Subtract => Instruction::Subtract(Subtract {}),
      TokenType::Multiply => Instruction::Multiply(Multiply {}),
      TokenType::Divide => Instruction::Divide(Divide {}),
      TokenType::Modulo => Instruction::Modulo(Modulo {}),
      TokenType::LeftShift => Instruction::LeftShift(LeftShift {}),
      TokenType::RightShift => Instruction::RightShift(RightShift {}),
      TokenType::BitwiseAnd => Instruction::BitwiseAnd(BitwiseAnd {}),
      TokenType::BitwiseOr => Instruction::BitwiseOr(BitwiseOr {}),
      TokenType::BitwiseXor => Instruction::BitwiseXor(BitwiseXor {}),
      TokenType::BitwiseNot => Instruction::BitwiseNot(BitwiseNot {}),
      TokenType::And => Instruction::And(And {}),
      TokenType::Or => Instruction::Or(Or {}),
      TokenType::Xor => Instruction::Xor(Xor {}),
      TokenType::Not => Instruction::Not(Not {}),
      TokenType::Equal => Instruction::Equal(Equal {}),
      TokenType::NotEqual => Instruction::NotEqual(NotEqual {}),
      TokenType::LessThan => Instruction::LessThan(LessThan {}),
      TokenType::GreaterThan => Instruction::GreaterThan(GreaterThan {}),
      TokenType::LessThanEqual => Instruction::LessThanEqual(LessThanEqual {}),
      TokenType::GreaterThanEqual => Instruction::GreaterThanEqual(GreaterThanEqual {}),
      TokenType::Jump => {
        let number_token = self.expect_token_type(TokenType::Number);
        self.consume();

        Instruction::Jump(Jump {
          index: number_token.lexeme.parse::<usize>().unwrap(),
        })
      }
      TokenType::Branch => {
        let true_token = self.expect_token_type(TokenType::Number);
        self.consume();

        let false_token = self.expect_token_type(TokenType::Number);
        self.consume();

        Instruction::Branch(Branch {
          true_index: true_token.lexeme.parse::<usize>().unwrap(),
          false_index: false_token.lexeme.parse::<usize>().unwrap(),
        })
      }
      TokenType::Call => Instruction::Call(Call {}),
      TokenType::Return => Instruction::Return(Return {}),
      TokenType::Load => Instruction::Load(Load {}),
      TokenType::Store => Instruction::Store(Store {}),
      TokenType::Index => {
        let optional_id = self.optional_token_type(TokenType::Identifier);
        match optional_id {
          Some(token) => {
            self.consume();
            Instruction::LayoutIndex(LayoutIndex {
              member: token.lexeme,
            })
          }
          None => Instruction::ArrayIndex(ArrayIndex {}),
        }
      }
      _ => panic!("Unexpected token. Expected an instruction :("),
    }
  }

  fn parse_reference(&mut self) -> Reference {
    let ref_token = self.expect_token();
    self.consume();

    match ref_token.token_type {
      TokenType::VariableRef => {
        let id_token = self.expect_token_type(TokenType::Identifier);
        self.consume();

        Reference::Variable(VariableRef {
          name: id_token.lexeme,
        })
      }
      TokenType::IndexRef => {
        let reference = self.parse_reference();

        let id_token = self.expect_token_type(TokenType::Identifier);
        self.consume();

        Reference::LayoutIndex(LayoutIndexRef {
          reference: Box::new(Value::Reference(reference)),
          index: id_token.lexeme.clone(),
        })
      }
      _ => panic!("Unexpected token here :("),
    }
  }

  fn parse_layout(&mut self) -> LayoutTemplate {
    let _ = self.expect_token_type(TokenType::Layout);
    self.consume();

    let identifier_token = self.expect_token_type(TokenType::Identifier);
    self.consume();

    let mut layout_template = LayoutTemplate::build(identifier_token.lexeme.as_str());

    loop {
      let type_token = self.expect_token();
      match type_token.token_type {
        TokenType::Type => self.consume(),
        TokenType::Module | TokenType::Function | TokenType::Layout | TokenType::Using => break,
        _ => panic!("Expected to have a type token here :("),
      }

      let identifier_token = self.expect_token_type(TokenType::Identifier);
      self.consume();

      layout_template = layout_template.member(
        identifier_token.lexeme.as_str(),
        Parser::create_default_value_from_type_string(type_token.lexeme),
      )
    }

    layout_template
  }

  fn create_default_value_from_type_string(type_lexeme: String) -> Value {
    match type_lexeme.as_str() {
      "bool" => Value::Boolean(false),
      "string" => Value::Array(Array::new(Box::new(Value::Unsigned8(0)))),
      "u8" => Value::Unsigned8(0),
      "u16" => Value::Unsigned16(0),
      "u32" => Value::Unsigned32(0),
      "u64" => Value::Unsigned64(0),
      "u128" => Value::Unsigned128(0),
      "s8" => Value::Signed8(0),
      "s16" => Value::Signed16(0),
      "s32" => Value::Signed32(0),
      "s64" => Value::Signed64(0),
      "s128" => Value::Signed128(0),
      _ => panic!("Unexpected type string"),
    }
  }

  pub fn create_value_from_type_string(
    type_lexeme: String,
    value_lexeme: String,
  ) -> Result<Value, String> {
    match type_lexeme.as_str() {
      "bool" => match value_lexeme.to_lowercase().as_str() {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        _ => Err("Unexpected value for boolean type".to_string()),
      },
      "string" => {
        let bytes = value_lexeme
          .clone()
          .into_bytes()
          .iter()
          .skip(1)
          .take(value_lexeme.len() - 2)
          .map(|x| Value::Unsigned8(*x))
          .collect::<Vec<Value>>();
        Ok(Value::Array(Array::create(
          Box::new(Value::Unsigned64(value_lexeme.len() as u64)),
          bytes,
        )))
      }
      "u8" => match value_lexeme.parse::<u8>() {
        Ok(value) => Ok(Value::Unsigned8(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a u8", value_lexeme)),
      },
      "u16" => match value_lexeme.parse::<u16>() {
        Ok(value) => Ok(Value::Unsigned16(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a u16", value_lexeme)),
      },
      "u32" => match value_lexeme.parse::<u32>() {
        Ok(value) => Ok(Value::Unsigned32(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a u32", value_lexeme)),
      },
      "u64" => match value_lexeme.parse::<u64>() {
        Ok(value) => Ok(Value::Unsigned64(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a u64", value_lexeme)),
      },
      "u128" => match value_lexeme.parse::<u128>() {
        Ok(value) => Ok(Value::Unsigned128(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a u128", value_lexeme)),
      },
      "s8" => match value_lexeme.parse::<i8>() {
        Ok(value) => Ok(Value::Signed8(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a s8", value_lexeme)),
      },
      "s16" => match value_lexeme.parse::<i16>() {
        Ok(value) => Ok(Value::Signed16(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a s16", value_lexeme)),
      },
      "s32" => match value_lexeme.parse::<i32>() {
        Ok(value) => Ok(Value::Signed32(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a s32", value_lexeme)),
      },
      "s64" => match value_lexeme.parse::<i64>() {
        Ok(value) => Ok(Value::Signed64(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a s64", value_lexeme)),
      },
      "s128" => match value_lexeme.parse::<i128>() {
        Ok(value) => Ok(Value::Signed128(value)),
        Err(_) => Err(format!("Couldn't parse '{}' into a s128", value_lexeme)),
      },
      _ => Err("Unexpected type string".to_string()),
    }
  }

  fn is_not_done(&self) -> bool {
    self.current_index < self.file_contents.len()
  }

  fn is_done(&self) -> bool {
    !self.is_not_done()
  }

  fn token(&mut self) -> Option<Token> {
    match &self.current_token {
      Some(current_token) => Some(current_token.clone()),
      None => {
        if self.is_done() {
          return None;
        }

        let mut lexeme = String::new();
        let start_index = self.current_index;
        let start_line = self.current_line;
        let start_column = self.current_column;

        let mut current_char = self.file_contents[self.current_index];
        // build lexeme
        while self.current_index < self.file_contents.len() {
          match current_char {
            'a'..='z' | 'A'..='Z' | '_' | '-' | '.' | '/' | '0'..='9' => {
              lexeme += &current_char.to_string();
              self.current_index += 1;
              self.current_column += 1;
            }
            '%' => {
              if lexeme.len() != 0 {
                break;
              }
              while current_char != '\n' && self.is_not_done() {
                lexeme += &current_char.to_string();
                self.current_index += 1;
                self.current_column += 1;
                if current_char == '\n' {
                  self.current_line += 1;
                  self.current_column = 1;
                }
                if self.is_not_done() {
                  current_char = self.file_contents[self.current_index];
                }
              }
            }
            '"' | '\'' => {
              if lexeme.len() != 0 {
                break;
              }
              let start_char = current_char;
              lexeme += &current_char.to_string();
              self.current_index += 1;
              self.current_column += 1;
              if self.is_not_done() {
                current_char = self.file_contents[self.current_index];
              }
              while current_char != start_char && self.is_not_done() {
                lexeme += &current_char.to_string();
                self.current_index += 1;
                self.current_column += 1;
                if current_char == '\n' {
                  self.current_line += 1;
                  self.current_column = 1;
                }
                if self.is_not_done() {
                  current_char = self.file_contents[self.current_index];
                }
              }
              if self.is_not_done() && current_char == start_char {
                lexeme += &current_char.to_string();
                self.current_index += 1;
                self.current_column += 1;
              }
            }
            _ => {
              if lexeme.len() == 0 {
                lexeme += &current_char.to_string();
                self.current_index += 1;
                self.current_column += 1;
              }
              break;
            }
          }
          if self.is_not_done() {
            current_char = self.file_contents[self.current_index];
          }
        }

        let final_index = self.current_index;
        let final_line = self.current_line;
        let final_column = self.current_column;

        // consume whitespace
        if self.is_not_done() {
          current_char = self.file_contents[self.current_index];
          while current_char.is_whitespace() && self.is_not_done() {
            self.current_index += 1;
            self.current_column += 1;
            if current_char == '\n' {
              self.current_line += 1;
              self.current_column = 1;
            }
            if self.is_not_done() {
              current_char = self.file_contents[self.current_index];
            }
          }
        }

        let number_re = Regex::new(r"^-?(([0-9]+)|([0-9]*\.[0-9]+))$").unwrap();
        let comment_re = Regex::new(r"^%.*$").unwrap();
        let identifier_re = Regex::new(r"^[0-9a-zA-Z.\-_\\/]+$").unwrap();
        let string_re = Regex::new(r#"^['"].*['"]$"#).unwrap();

        let token_type = match lexeme.to_lowercase().as_str() {
          "u8" | "u16" | "u32" | "u64" | "u128" => TokenType::Type,
          "s8" | "s16" | "s32" | "s64" | "s128" => TokenType::Type,
          "string" => TokenType::Type,
          "bool" => TokenType::Type,
          "true" => TokenType::True,
          "false" => TokenType::False,
          "main" => TokenType::Main,
          "funcp" => TokenType::FunctionPointer,
          "vref" => TokenType::VariableRef,
          "iref" => TokenType::IndexRef,
          "module" => TokenType::Module,
          "using" => TokenType::Using,
          "function" => TokenType::Function,
          "body" => TokenType::Body,
          "layout" => TokenType::Layout,
          "array" => TokenType::Array,
          "this" => TokenType::This,
          "alloc" => TokenType::Alloc,
          "push" => TokenType::Push,
          "pop" => TokenType::Pop,
          "duplicate" => TokenType::Duplicate,
          "swap" => TokenType::Swap,
          "add" => TokenType::Add,
          "subtract" => TokenType::Subtract,
          "multiply" => TokenType::Multiply,
          "divide" => TokenType::Divide,
          "modulo" => TokenType::Modulo,
          "shiftleft" => TokenType::LeftShift,
          "shiftright" => TokenType::RightShift,
          "bitand" => TokenType::BitwiseAnd,
          "bitor" => TokenType::BitwiseOr,
          "bitxor" => TokenType::BitwiseXor,
          "bitnot" => TokenType::BitwiseNot,
          "and" => TokenType::And,
          "or" => TokenType::Or,
          "xor" => TokenType::Xor,
          "not" => TokenType::Not,
          "equal" => TokenType::Equal,
          "notequal" => TokenType::NotEqual,
          "lessthan" => TokenType::LessThan,
          "lessthanequal" => TokenType::LessThanEqual,
          "greaterthan" => TokenType::GreaterThan,
          "greaterthanequal" => TokenType::GreaterThanEqual,
          "jump" => TokenType::Jump,
          "branch" => TokenType::Branch,
          "call" => TokenType::Call,
          "return" => TokenType::Return,
          "load" => TokenType::Load,
          "store" => TokenType::Store,
          "index" => TokenType::Index,
          _ => {
            if number_re.is_match(lexeme.as_str()) {
              TokenType::Number
            } else if comment_re.is_match(lexeme.as_str()) {
              TokenType::Comment
            } else if identifier_re.is_match(lexeme.as_str()) {
              TokenType::Identifier
            } else if string_re.is_match(lexeme.as_str()) {
              TokenType::String
            } else {
              TokenType::Error
            }
          }
        };

        match token_type {
          TokenType::Comment => {
            // todo this feels kinda odd but it skips over comments
            self.token();
          }
          _ => {
            self.current_token = Some(Token::new(
              lexeme,
              token_type,
              (start_index, final_index),
              (start_line, final_line),
              (start_column, final_column),
            ));
          }
        }

        self.current_token.clone()
      }
    }
  }

  fn expect_token(&mut self) -> Token {
    match self.token() {
      Some(token) => token,
      None => panic!("Expected a token here!"),
    }
  }

  fn expect_token_type(&mut self, token_type: TokenType) -> Token {
    match self.token() {
      Some(token) => {
        if token.is_token_type(token_type) {
          token
        } else {
          panic!(
            "Expected token type {:?} but got {:?}",
            token_type, token.token_type
          );
        }
      }
      None => panic!("Expected some token here :("),
    }
  }

  fn optional_token_type(&mut self, token_type: TokenType) -> Option<Token> {
    match self.token() {
      Some(token) => {
        if token.is_token_type(token_type) {
          Some(token)
        } else {
          None
        }
      }
      None => None,
    }
  }

  fn expect_one_of(&mut self, token_types: Vec<TokenType>) -> Token {
    match self.token() {
      Some(token) => {
        if token_types.contains(&token.token_type) {
          token
        } else {
          panic!(
            "Expected one of {:?} but got {:?}",
            token_types, token.token_type
          );
        }
      }
      None => panic!("Expected some token here :("),
    }
  }

  fn consume(&mut self) {
    self.current_token = None
  }
}
