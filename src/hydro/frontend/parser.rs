use crate::hydro::frontend::token::{Token, TokenType};
use crate::hydro::function::Function;
use crate::hydro::instruction::{
  Add, AllocLayout, And, BitwiseAnd, BitwiseNot, BitwiseOr, BitwiseXor, Branch, Call, Divide,
  Equal, GreaterThan, GreaterThanEqual, Index, Instruction, Jump, LeftShift, LessThan,
  LessThanEqual, Load, Modulo, Multiply, Not, NotEqual, Or, PopValue, PushValue, Return,
  RightShift, Store, Subtract, Xor,
};
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::module::Module;
use crate::hydro::value::{FunctionPointer, IndexRef, Reference, Value, VariableRef};
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
    let Some(module_token) = self.token() else {
      panic!("Expected to have a token here :(");
    };
    match module_token.token_type {
      TokenType::Module => self.consume(),
      _ => panic!("Expected to have a module token here :("),
    }

    let Some(identifier_token) = self.token() else {
      panic!("Expected to have a token here :(");
    };
    match identifier_token.token_type {
      TokenType::Identifier => self.consume(),
      TokenType::Main => self.consume(),
      _ => panic!("Expected to have an identifier token or main here :("),
    }

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
    let Some(using_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };
    match using_token.token_type {
      TokenType::Using => self.consume(),
      _ => panic!("Expected to have a using token here :("),
    }

    let Some(identifier_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };
    match identifier_token.token_type {
      TokenType::Identifier => self.consume(),
      _ => panic!("Expected to have an identifier token here :("),
    }

    identifier_token.lexeme
  }

  fn parse_function(&mut self) -> Function {
    let Some(function_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };
    match function_token.token_type {
      TokenType::Function => self.consume(),
      TokenType::Main => { /* DONT CONSUME HERE BECAUSE WE NEED TO GET THE MAIN TOKEN FOR THE IDENTIFIER */
      }
      _ => panic!("Expected to have a function token here :("),
    }

    let Some(identifier_token) = self.token() else {
      panic!("Expected to have a token here :(");
    };
    match identifier_token.token_type {
      TokenType::Identifier => self.consume(),
      TokenType::Main => self.consume(),
      _ => panic!("Expected to have an identifier token or main here :("),
    }

    let mut function = Function::build(identifier_token.lexeme.as_str());
    // parse params
    loop {
      let Some(id_token) = self.token() else {
        panic!("Expected to have a token here :(")
      };
      match id_token.token_type {
        TokenType::Identifier => self.consume(),
        TokenType::Body => break,
        _ => panic!("Expected to have a using token here :("),
      }
      function = function.parameter(id_token.lexeme.as_str());
    }

    let Some(body_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };
    match body_token.token_type {
      TokenType::Body => self.consume(),
      _ => panic!("Expected to have a body token here :("),
    }

    // parse insts
    loop {
      let Some(inst_token) = self.token() else {
        break
      };
      match inst_token.token_type {
        TokenType::Alloc
        | TokenType::Push
        | TokenType::Pop
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
        _ => panic!("Expected to have a using token here :("),
      }
    }

    function
  }

  fn parse_instruction(&mut self) -> Instruction {
    let Some(inst_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };

    match inst_token.token_type {
      TokenType::Alloc => {
        self.consume();

        let Some(alloc_type_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };

        match alloc_type_token.token_type {
          TokenType::Layout => {
            self.consume();

            let Some(module_token) = self.token() else {
              panic!("Expected to have a token here :(");
            };
            match module_token.token_type {
              TokenType::Identifier => self.consume(),
              TokenType::This => self.consume(),
              _ => panic!("Expected to have an identifier token here :("),
            }

            let Some(layout_template_token) = self.token() else {
              panic!("Expected to have a token here :(");
            };
            match layout_template_token.token_type {
              TokenType::Identifier => self.consume(),
              _ => panic!("Expected to have an identifier token here :("),
            }

            Instruction::AllocLayout(AllocLayout {
              module_name: match module_token.token_type {
                TokenType::Identifier => Some(module_token.lexeme),
                TokenType::This => None,
                _ => panic!("Shouldn't be hit"),
              },
              layout_template_name: layout_template_token.lexeme,
            })
          }
          TokenType::Array => todo!(),
          _ => panic!("Expected to have a type token here :("),
        }
      }
      TokenType::Push => {
        self.consume();

        let Some(type_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };
        match type_token.token_type {
          TokenType::Type => self.consume(),
          TokenType::VariableRef => { /*DONT CONSUME*/ }
          TokenType::IndexRef => { /*DONT CONSUME*/ }
          TokenType::FunctionPointer => { /*DONT CONSUME*/ }
          TokenType::Identifier => { /*DONT CONSUME*/ }
          _ => panic!("Expected to have a type token here :( {:?}", type_token),
        }

        let Some(value_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };
        Instruction::PushValue(PushValue {
          value: match value_token.token_type {
            TokenType::Number => {
              self.consume();
              Parser::create_value_from_type_string(type_token.lexeme, value_token.lexeme.clone())
            }
            TokenType::True => {
              self.consume();
              Value::Boolean(true)
            }
            TokenType::False => {
              self.consume();
              Value::Boolean(true)
            }
            TokenType::VariableRef | TokenType::IndexRef => {
              Value::Reference(self.parse_reference())
            }
            TokenType::FunctionPointer => {
              self.consume();

              let Some(module_token) = self.token() else {
                panic!("Expected to have a token here :(");
              };
              match module_token.token_type {
                TokenType::Identifier => self.consume(),
                TokenType::This => self.consume(),
                _ => panic!("Expected to have an identifier token here :("),
              }

              let Some(function_token) = self.token() else {
                panic!("Expected to have a token here :(");
              };
              match function_token.token_type {
                TokenType::Identifier => self.consume(),
                _ => panic!("Expected to have an identifier token here :("),
              }

              Value::FunctionPointer(FunctionPointer {
                module: match module_token.token_type {
                  TokenType::Identifier => Some(module_token.lexeme),
                  TokenType::This => None,
                  _ => panic!("Shouldn't be hit"),
                },
                function: function_token.lexeme,
              })
            }
            TokenType::Identifier => {
              self.consume();
              Value::String(value_token.lexeme)
            }
            _ => panic!("Expected to have a value token here :("),
          },
        })
      }
      TokenType::Pop => {
        self.consume();
        Instruction::PopValue(PopValue {})
      }
      TokenType::Add => {
        self.consume();
        Instruction::Add(Add {})
      }
      TokenType::Subtract => {
        self.consume();
        Instruction::Subtract(Subtract {})
      }
      TokenType::Multiply => {
        self.consume();
        Instruction::Multiply(Multiply {})
      }
      TokenType::Divide => {
        self.consume();
        Instruction::Divide(Divide {})
      }
      TokenType::Modulo => {
        self.consume();
        Instruction::Modulo(Modulo {})
      }
      TokenType::LeftShift => {
        self.consume();
        Instruction::LeftShift(LeftShift {})
      }
      TokenType::RightShift => {
        self.consume();
        Instruction::RightShift(RightShift {})
      }
      TokenType::BitwiseAnd => {
        self.consume();
        Instruction::BitwiseAnd(BitwiseAnd {})
      }
      TokenType::BitwiseOr => {
        self.consume();
        Instruction::BitwiseOr(BitwiseOr {})
      }
      TokenType::BitwiseXor => {
        self.consume();
        Instruction::BitwiseXor(BitwiseXor {})
      }
      TokenType::BitwiseNot => {
        self.consume();
        Instruction::BitwiseNot(BitwiseNot {})
      }
      TokenType::And => {
        self.consume();
        Instruction::And(And {})
      }
      TokenType::Or => {
        self.consume();
        Instruction::Or(Or {})
      }
      TokenType::Xor => {
        self.consume();
        Instruction::Xor(Xor {})
      }
      TokenType::Not => {
        self.consume();
        Instruction::Not(Not {})
      }
      TokenType::Equal => {
        self.consume();
        Instruction::Equal(Equal {})
      }
      TokenType::NotEqual => {
        self.consume();
        Instruction::NotEqual(NotEqual {})
      }
      TokenType::LessThan => {
        self.consume();
        Instruction::LessThan(LessThan {})
      }
      TokenType::GreaterThan => {
        self.consume();
        Instruction::GreaterThan(GreaterThan {})
      }
      TokenType::LessThanEqual => {
        self.consume();
        Instruction::LessThanEqual(LessThanEqual {})
      }
      TokenType::GreaterThanEqual => {
        self.consume();
        Instruction::GreaterThanEqual(GreaterThanEqual {})
      }
      TokenType::Jump => {
        self.consume();

        let Some(number_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };
        match number_token.token_type {
          TokenType::Number => self.consume(),
          _ => panic!("Expected to have a type token here :("),
        }

        Instruction::Jump(Jump {
          index: number_token.lexeme.parse::<usize>().unwrap(),
        })
      }
      TokenType::Branch => {
        self.consume();

        let Some(true_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };
        match true_token.token_type {
          TokenType::Number => self.consume(),
          _ => panic!("Expected to have a number token here :("),
        }

        let Some(false_token) = self.token() else {
          panic!("Expected to have a token here :(")
        };
        match false_token.token_type {
          TokenType::Number => self.consume(),
          _ => panic!("Expected to have a number token here :("),
        }

        Instruction::Branch(Branch {
          true_index: true_token.lexeme.parse::<usize>().unwrap(),
          false_index: false_token.lexeme.parse::<usize>().unwrap(),
        })
      }
      TokenType::Call => {
        self.consume();
        Instruction::Call(Call {})
      }
      TokenType::Return => {
        self.consume();
        Instruction::Return(Return {})
      }
      TokenType::Load => {
        self.consume();
        Instruction::Load(Load {})
      }
      TokenType::Store => {
        self.consume();
        Instruction::Store(Store {})
      }
      TokenType::Index => {
        self.consume();
        Instruction::Index(Index {})
      }
      _ => panic!("Unexpected token. Expected an instruction :("),
    }
  }

  fn parse_reference(&mut self) -> Reference {
    let Some(ref_token) = self.token() else {
       panic!("Expected to have a token here :(")
     };

    match ref_token.token_type {
      TokenType::VariableRef => {
        self.consume();

        let Some(id_token) = self.token() else {
           panic!("Expected to have a token here :(")
         };
        match id_token.token_type {
          TokenType::Identifier => self.consume(),
          _ => panic!("Expected to have an identifier token here :("),
        }

        Reference::Variable(VariableRef {
          name: id_token.lexeme,
        })
      }
      TokenType::IndexRef => {
        self.consume();

        let reference = self.parse_reference();

        let Some(id_token) = self.token() else {
           panic!("Expected to have a token here :(")
         };
        match id_token.token_type {
          TokenType::Identifier => self.consume(),
          _ => panic!("Expected to have an identifier token here :("),
        }

        Reference::Index(IndexRef {
          reference: Box::new(Value::Reference(reference)),
          index: Box::new(Value::String(id_token.lexeme)),
        })
      }
      _ => panic!("Unexpected token here :("),
    }
  }

  fn parse_layout(&mut self) -> LayoutTemplate {
    let Some(layout_token) = self.token() else {
      panic!("Expected to have a token here :(")
    };
    match layout_token.token_type {
      TokenType::Layout => self.consume(),
      _ => panic!("Expected to have a layout token here :("),
    }

    let Some(identifier_token) = self.token() else {
      panic!("Expected to have a token here :(");
    };
    match identifier_token.token_type {
      TokenType::Identifier => self.consume(),
      _ => panic!("Expected to have an identifier token here :("),
    }

    let mut layout_template = LayoutTemplate::build(identifier_token.lexeme.as_str());

    loop {
      let Some(type_token) = self.token() else {
        panic!("Expected to have a token here :(")
      };
      match type_token.token_type {
        TokenType::Type => self.consume(),
        TokenType::Module | TokenType::Function | TokenType::Layout | TokenType::Using => break,
        _ => panic!("Expected to have a type token here :("),
      }

      let Some(identifier_token) = self.token() else {
        panic!("Expected to have a token here :(")
      };
      match identifier_token.token_type {
        TokenType::Identifier => self.consume(),
        _ => panic!("Expected to have an identifier token here :("),
      }
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
      "string" => Value::String("".to_string()),
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

  fn create_value_from_type_string(type_lexeme: String, value_lexeme: String) -> Value {
    match type_lexeme.as_str() {
      "bool" => Value::Boolean(match value_lexeme.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("Unexpected value for boolean type"),
      }),
      "string" => Value::String(value_lexeme),
      "u8" => Value::Unsigned8(value_lexeme.parse::<u8>().unwrap()),
      "u16" => Value::Unsigned16(value_lexeme.parse::<u16>().unwrap()),
      "u32" => Value::Unsigned32(value_lexeme.parse::<u32>().unwrap()),
      "u64" => Value::Unsigned64(value_lexeme.parse::<u64>().unwrap()),
      "u128" => Value::Unsigned128(value_lexeme.parse::<u128>().unwrap()),
      "s8" => Value::Signed8(value_lexeme.parse::<i8>().unwrap()),
      "s16" => Value::Signed16(value_lexeme.parse::<i16>().unwrap()),
      "s32" => Value::Signed32(value_lexeme.parse::<i32>().unwrap()),
      "s64" => Value::Signed64(value_lexeme.parse::<i64>().unwrap()),
      "s128" => Value::Signed128(value_lexeme.parse::<i128>().unwrap()),
      _ => panic!("Unexpected type string"),
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

  fn consume(&mut self) {
    self.current_token = None
  }
}
