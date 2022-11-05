use std::fmt;
use std::str;

const NEW_LINE: u8 = b'\n';
const LINE_FEED: u8 = b'\r';

#[derive(Debug)]
pub struct TokenLocation {
  row: usize,
  col: usize,
}

#[derive(Debug)]
pub enum Operators {
  Plus,
  Minus,
  Star,
  Assignment,
  Increment,
  Decrement,
}

pub enum Literals<'a> {
  String(&'a [u8]),
  Number(&'a [u8]),
}

impl<'a> fmt::Debug for Literals<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Literals::String(bytes) => {
        write!(f, "\"{}\"", str::from_utf8(bytes).unwrap())
      }
      Literals::Number(bytes) => {
        write!(f, "{}", str::from_utf8(bytes).unwrap())
      }
    }
  }
}

#[derive(Debug)]
pub enum Token<'a> {
  Operator(TokenLocation, Operators),
  OpenBrace(TokenLocation),
  CloseBrace(TokenLocation),
  OpenParen(TokenLocation),
  CloseParen(TokenLocation),
  Literal(TokenLocation, Literals<'a>),
  EOF(TokenLocation),
}

pub struct Lexer<'a> {
  row: usize,
  col: usize,
  current: usize,
  code_bytes: &'a [u8],
  tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new() -> Self {
    Self {
      row: 1,
      col: 1,
      current: 0,
      tokens: vec![],
      code_bytes: &[],
    }
  }

  fn peek(&self) -> u8 {
    self.code_bytes[self.current + 1]
  }

  fn lookup(&mut self, lookup_char: u8) -> bool {
    if !self.is_eof(1) {
      let lookup_matched = self.peek() == lookup_char;

      if lookup_matched {
        self.advance();
      }

      return lookup_matched;
    }

    return false;
  }

  fn get_current_char_byte(&self) -> u8 {
    self.code_bytes[self.current]
  }

  fn advance(&mut self) {
    if self.is_eol() {
      self.row +=1;
      self.col = 1;
    } else {
      self.col += 1;
    }

    self.current += 1;
  }

  fn is_eof(&self, offset: usize) -> bool {
    self.current + offset >= self.code_bytes.len()
  }

  fn is_eol(&self) -> bool {
    !self.is_eof(0) && (self.get_current_char_byte() == NEW_LINE || self.get_current_char_byte() == LINE_FEED)
  }

  fn eat_string(&mut self) {
    let str_start_col = self.col;
    let str_start_row = self.row;
    self.advance();
    let str_start = self.current;

    while !self.is_eof(0)
      && self.get_current_char_byte() != b'"'
      && !self.is_eol()
    {
      self.advance();
    }

    if self.current == self.code_bytes.len() || self.get_current_char_byte() != b'"' {
      panic!(
        "non terminated string found at {}:{}",
        str_start_row, str_start_col
      )
    } else {
      let str_bytes = &self.code_bytes[str_start..self.current];

      self.tokens.push(Token::Literal(
        self.get_current_token_location(),
        Literals::String(str_bytes),
      ))
    }
  }

  fn eat_number(&mut self) {
    let num_start_col = self.col;
    let num_start_row = self.row;
    let num_start = self.current;
    let mut is_decimal_point_eaten = false;

    while !self.is_eof(1) && self.is_digit(self.peek())
    {  
      if self.peek() == b'.' {
        if !is_decimal_point_eaten {
          is_decimal_point_eaten = true;
        } else {
          break;
        }
      }

      self.advance();
    }

    let num_bytes = &self.code_bytes[num_start..self.current + 1];

    self.tokens.push(Token::Literal(
      TokenLocation {
        row: num_start_row,
        col: num_start_col,
      },
      Literals::Number(num_bytes),
    ))
  }

  fn eat_hash_single_line_comment(&mut self) {
    while !self.is_eof(0) && !self.is_eol() {
      self.advance();
    }
  }

  fn is_digit(&self, character: u8) -> bool {
    (character >= b'0' && character <= b'9') || character == b'.'
  }

  fn get_current_token_location(&self) -> TokenLocation {
    TokenLocation {
      row: self.row,
      col: self.col,
    }
  }

  pub fn lex(&mut self, code: &'a str) -> &Vec<Token> {
    self.tokens = vec![];
    self.current = 0;
    self.code_bytes = code.as_bytes();

    while self.current < code.len() {
      let char_string = code.get(self.current..self.current + 1).unwrap();
      match self.get_current_char_byte() {
        b' ' => (),
        NEW_LINE | LINE_FEED => {
          self.advance();
        }
        b'+' => {
          if self.lookup(b'+') {
            self.tokens.push(Token::Operator(
              self.get_current_token_location(),
              Operators::Increment,
            ))
          } else {
            self.tokens.push(Token::Operator(
              self.get_current_token_location(),
              Operators::Plus,
            ))
          }
        }
        b'-' => {
          if self.lookup(b'-') {
            self.tokens.push(Token::Operator(
              self.get_current_token_location(),
              Operators::Decrement,
            ))
          } else {
            self.tokens.push(Token::Operator(
              self.get_current_token_location(),
              Operators::Minus,
            ))
          }
        }
        b'*' => self.tokens.push(Token::Operator(
          self.get_current_token_location(),
          Operators::Star,
        )),
        b'{' => self
          .tokens
          .push(Token::OpenBrace(self.get_current_token_location())),
        b'}' => self
          .tokens
          .push(Token::CloseBrace(self.get_current_token_location())),
        b'(' => self
          .tokens
          .push(Token::OpenParen(self.get_current_token_location())),
        b')' => self
          .tokens
          .push(Token::CloseParen(self.get_current_token_location())),
        b'=' => self.tokens.push(Token::Operator(
          self.get_current_token_location(),
          Operators::Assignment,
        )),
        b'"' => self.eat_string(),
        b'#' => self.eat_hash_single_line_comment(),
        b'0'..=b'9' => self.eat_number(),
        _ => panic!(
          "invalid token {} found at {}:{}",
          char_string, self.row, self.col
        )
      }
      self.advance();
    }
    self.tokens.push(Token::EOF(self.get_current_token_location()));

    &self.tokens
  }
}
