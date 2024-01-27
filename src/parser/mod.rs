use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
  Await,
  Break,
  Case,
  Catch,
  Class,
  Const,
  Continue,
  Debugger,
  Default,
  Delete,
  Do,
  Else,
  Enum,
  Export,
  Extends,
  False,
  Finally,
  For,
  Function,
  If,
  Implements,
  Import,
  In,
  Instanceof,
  Interface,
  Let,
  New,
  Null,
  Package,
  Private,
  Protected,
  Public,
  Return,
  Super,
  Static,
  Switch,
  This,
  Throw,
  True,
  Try,
  Typeof,
  Var,
  Void,
  While,
  With,
  Yield,
}

impl Keyword {
  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "await" => Some(Self::Await),
      "break" => Some(Self::Break),
      "case" => Some(Self::Case),
      "catch" => Some(Self::Catch),
      "class" => Some(Self::Class),
      "const" => Some(Self::Const),
      "continue" => Some(Self::Continue),
      "debugger" => Some(Self::Debugger),
      "default" => Some(Self::Default),
      "delete" => Some(Self::Delete),
      "do" => Some(Self::Do),
      "else" => Some(Self::Else),
      "enum" => Some(Self::Enum),
      "export" => Some(Self::Export),
      "extends" => Some(Self::Extends),
      "false" => Some(Self::False),
      "finally" => Some(Self::Finally),
      "for" => Some(Self::For),
      "function" => Some(Self::Function),
      "if" => Some(Self::If),
      "implements" => Some(Self::Implements),
      "import" => Some(Self::Import),
      "in" => Some(Self::In),
      "instanceof" => Some(Self::Instanceof),
      "interface" => Some(Self::Interface),
      "let" => Some(Self::Let),
      "new" => Some(Self::New),
      "null" => Some(Self::Null),
      "package" => Some(Self::Package),
      "private" => Some(Self::Private),
      "protected" => Some(Self::Protected),
      "public" => Some(Self::Public),
      "return" => Some(Self::Return),
      "super" => Some(Self::Super),
      "static" => Some(Self::Static),
      "switch" => Some(Self::Switch),
      "this" => Some(Self::This),
      "throw" => Some(Self::Throw),
      "true" => Some(Self::True),
      "try" => Some(Self::Try),
      "typeof" => Some(Self::Typeof),
      "var" => Some(Self::Var),
      "void" => Some(Self::Void),
      "while" => Some(Self::While),
      "with" => Some(Self::With),
      "yield" => Some(Self::Yield),
      _ => {
        None
      }
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType { 
  LeftParen,    // '('
  RightParen,   // ')'
  Semicolon,    // ';'
  Dot,          // '.'
  Asterisk,     // '*'
  Comma,        // ','
  Equals,       // '='
  Plus,         // '+'
  Minus,        // '-'
  Slash,        // '/'
  Ampersand,    // '&'
  Backslash,    // '\\'
  Pipe,         // '|'
  LeftBrace,    // '{'
  RightBrace,   // '}'
  LeftBracket,  // '['
  RightBracket, // ']'
  Percent,      // '%'
  Question,     // '?'
  Colon,        // ':'
  Space,        // ' '
  NewLine,      // '\n'
  SingleQuote,  // '\''
  DoubleQuote,  // '"'
  Backtick,     // '`'
  Underscore,   // '_'
  Keyword(Keyword),
  String,
  Number,
  Identifier,
  Unknown,
  Error(String)
}

impl TokenType {
  pub fn from_char(c: char) -> Self {
    match c {
      '(' => TokenType::LeftParen,
      ')' => TokenType::RightParen,
      ';' => TokenType::Semicolon,
      '.' => TokenType::Dot,
      '*' => TokenType::Asterisk,
      ',' => TokenType::Comma,
      '=' => TokenType::Equals,
      '+' => TokenType::Plus,
      '-' => TokenType::Minus,
      '/' => TokenType::Slash,
      '&' => TokenType::Ampersand,
      '\\' => TokenType::Backslash,
      '|' => TokenType::Pipe,
      '{' => TokenType::LeftBrace,
      '}' => TokenType::RightBrace,
      '[' => TokenType::LeftBracket,
      ']' => TokenType::RightBracket,
      '%' => TokenType::Percent,
      '?' => TokenType::Question,
      ':' => TokenType::Colon,
      ' ' => TokenType::Space,
      '\'' => TokenType::SingleQuote,
      '"' => TokenType::DoubleQuote,
      '`' => TokenType::Backtick,
      '_' => TokenType::Underscore,
      '\n' => TokenType::NewLine,
      _ => TokenType::Unknown,
    }
  }
}

// token을 먼저 잘개 쪼갠 뒤 그 다음 병합하여 의미 있는 구문을 만든다.
// token type은 처음에 unknown으로 만든 뒤 그 타입을 정해 준다.

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub value: String,
  pub line: usize,
  pub loc_start: usize,
  pub loc_end: usize,
  pub start: usize,
  pub end: usize
}

pub fn tokenize(input: &str) -> Vec<Token> {
  let mut tokens = Vec::<Token>::new();
  let mut word = String::new();
  let mut line_count: usize = 1;
  let mut word_count: usize = 0; 
  let mut loc_word_count: usize = 0;

  for c in input.chars() {
    let token_type: TokenType = TokenType::from_char(c);

    match token_type {
      TokenType::Unknown => {
        word.push(c);
        loc_word_count += 1;
      },
      TokenType::Space => {
        if word != "" {
          tokens.push(Token { 
            token_type: TokenType::Unknown,
            value: word.clone(),
            line: line_count,
            start: word_count - word.len(),
            end: word_count,
            loc_start: loc_word_count - word.len(),
            loc_end: loc_word_count,
          });
          word.clear();
        }
 
        loc_word_count += 1;
      },
      TokenType::NewLine => {
        if word != "" {
          tokens.push(Token { 
            token_type: TokenType::Unknown,
            value: word.clone(),
            line: line_count,
            start: word_count - word.len(),
            end: word_count,
            loc_start: loc_word_count - word.len(),
            loc_end: loc_word_count,
          });
          word.clear();
        }

        line_count += 1;
        loc_word_count = 0;
      },
      _ => {
        if word != "" {
          tokens.push(Token { 
            token_type: TokenType::Unknown,
            value: word.clone(),
            line: line_count,
            start: word_count - word.len(),
            end: word_count,
            loc_start: loc_word_count - word.len(),
            loc_end: loc_word_count,
          });
          word.clear();
        }

        tokens.push(Token { 
          token_type,
          value: c.to_string(),
          line: line_count,
          start: word_count,
          end: word_count,
          loc_start: loc_word_count,
          loc_end: loc_word_count + 1
        });

        loc_word_count += 1;
      }
    }

    word_count += 1;
  }

  tokens
}

fn merge_token_string<'a>(tokens: &mut Peekable<Iter<'a, Token>>, start_token: &Token) -> Token {
  if let Some(peek_token) = tokens.peek() {
    if peek_token.line > start_token.line {
      return Token {
        token_type: TokenType::Error("not string".to_string()),
        value: start_token.value.clone(),
        line: start_token.line,
        start: start_token.start,
        end: start_token.end,
        loc_start: start_token.loc_start,
        loc_end: start_token.loc_end,
      };
    }
  }
  
  let mut value: String = start_token.value.clone();
  let mut last_token: &Token = start_token;

  while let Some(token) = tokens.next() {
    let space_count: usize = token.loc_start - last_token.loc_end;
    let mut i: usize = 0;

    while i < space_count {
      value.push_str(" ");
      i += 1;
    }

    last_token = token;
    value.push_str(&token.value);

    if token.token_type == start_token.token_type {
      return Token {
        token_type: TokenType::String,
        value,
        line: start_token.line,
        start: start_token.start,
        end: token.end,
        loc_start: start_token.loc_start,
        loc_end: token.loc_end,
      }
    }

    if let Some(peek_token) = tokens.peek() {
      if peek_token.line > start_token.line {
        break;
      }
    }
  }

  Token {
    token_type: TokenType::Error("not string".to_string()),
    value,
    line: start_token.line,
    start: start_token.start,
    end: last_token.end,
    loc_start: start_token.loc_start,
    loc_end: last_token.loc_end,
  }
}

fn merge_token_number<'a>(tokens: &mut Peekable<Iter<'a, Token>>, start_token: &Token) -> Token {
  if let Some(peek_token) = tokens.peek() {
    if peek_token.line > start_token.line {
      return Token { 
        token_type: TokenType::Number,
        value: start_token.value.clone(),
        line: start_token.line,
        start: start_token.start,
        end: start_token.end,
        loc_start: start_token.loc_start,
        loc_end: start_token.loc_end,
      }
    }
  }
  
  let mut value: String = start_token.value.clone();
  let mut last_token: &Token = start_token;
  let mut token_type: TokenType = TokenType::Number;

  while let Some(token) = tokens.next() {
    let space_count: usize = token.loc_start - last_token.loc_end;
    let mut i: usize = 0;

    while i < space_count {
      value.push_str(" ");
      i += 1;
    }

    last_token = token;
    value.push_str(&token.value);

    if let Some(peek_token) = tokens.peek() {
      if peek_token.line > start_token.line || peek_token.token_type == TokenType::Semicolon {
        break;
      }
    }
  }

  if value.chars().all(char::is_numeric) == false {
    token_type = TokenType::Error("not number".to_string())
  } 

  Token { 
    token_type,
    value,
    line: start_token.line,
    start: start_token.start,
    end: last_token.end,
    loc_start: start_token.loc_start,
    loc_end: last_token.loc_end,
  }
}

fn merge_token(tokens: Vec<Token>) -> Vec<Token> {
  let mut merged_tokens: Vec<Token> = Vec::new();
  let mut t = tokens.iter().peekable();

  while let Some(token) = t.next() {
    match token.token_type {
      TokenType::DoubleQuote | TokenType::SingleQuote | TokenType::Backtick => {
        merged_tokens.push(merge_token_string(&mut t, token));
      },
      TokenType::Unknown => {

        let token_type: TokenType = {
          match Keyword::from_str(&token.value) {
            Some(keyword) => TokenType::Keyword(keyword),
            None => {
              if token.value.chars().all(char::is_numeric) {
                TokenType::Number
              } else {
                TokenType::Identifier
              }
            }
          }
        };

        if token_type == TokenType::Number {
          merged_tokens.push(merge_token_number(&mut t, token));
        } else {
          merged_tokens.push(Token {
            token_type,
            value: token.value.clone(),
            line: token.line,
            loc_start: token.loc_start,
            loc_end: token.loc_end,
            start: token.start,
            end: token.end,
          });
        }
      },
      _ => {
        merged_tokens.push(token.clone());
      }
    }
  }

  merged_tokens
}

pub fn parse_source(input: &str) -> Vec<Token> {
  merge_token(tokenize(input))
}