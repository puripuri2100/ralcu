use super::types;

// トークン
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenKind {
  EOF,
  VAR(String),
  CONSTRUCTOR(String),
  INTCONST(i64),
  FLOATCONST(f64),
  DEF_EQ,
  BINOP_TIMES(String),
  BINOP_DIVIDES(String),
  BINOP_PLUS(String),
  BINOP_MINUS(String),
  BINOP_HAT(String),
  BINOP_AMP(String),
  BINOP_BAR(String),
  BINOP_GT(String),
  BINOP_LT(String),
  BINOP_EQ(String),
  LPAREN,
  RPAREN,
  LBRACKET,
  RBRACKET,
  SEMICOLON,
  COLON,
  COMMA,
  TRUE,
  FALSE,
  IF,
  THEN,
  ELSE,
  FN,
  LET,
  MUT,
  WHILE,
}

// 位置情報とのタプルで表す
pub type Token = (TokenKind, types::Range);

pub fn get_string(tok: TokenKind) -> Option<String> {
  match tok {
    TokenKind::VAR(s) => Some(s),
    TokenKind::CONSTRUCTOR(s) => Some(s),
    TokenKind::BINOP_TIMES(s) => Some(s),
    TokenKind::BINOP_DIVIDES(s) => Some(s),
    TokenKind::BINOP_PLUS(s) => Some(s),
    TokenKind::BINOP_MINUS(s) => Some(s),
    TokenKind::BINOP_HAT(s) => Some(s),
    TokenKind::BINOP_AMP(s) => Some(s),
    TokenKind::BINOP_BAR(s) => Some(s),
    TokenKind::BINOP_GT(s) => Some(s),
    TokenKind::BINOP_LT(s) => Some(s),
    TokenKind::BINOP_EQ(s) => Some(s),
    _ => None,
  }
}

pub fn get_i64(tok: TokenKind) -> Option<i64> {
  match tok {
    TokenKind::INTCONST(i) => Some(i),
    _ => None,
  }
}

pub fn get_f64(tok: TokenKind) -> Option<f64> {
  match tok {
    TokenKind::FLOATCONST(f) => Some(f),
    _ => None,
  }
}

// エラー情報の実装
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
  InvalidChar(char),
  UnDefinedToken(String),
  Eof,
}

pub type LexError = (LexErrorKind, types::Range);

#[allow(dead_code)]
fn error_invalid_char(c: char, r: types::Range) -> LexError {
  (LexErrorKind::InvalidChar(c), r)
}

#[allow(dead_code)]
fn error_undefined_token(s: String, r: types::Range) -> LexError {
  (LexErrorKind::UnDefinedToken(s), r)
}

#[allow(dead_code)]
fn error_eof(r: types::Range) -> LexError {
  (LexErrorKind::Eof, r)
}

fn is_digit(c: &char) -> bool {
  matches!(
    *c,
    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
  )
}
fn is_capital(c: &char) -> bool {
  matches!(*c, 'A'..='Z')
}
fn is_small(c: &char) -> bool {
  matches!(*c, 'a'..='z')
}
fn is_opsymbol(c: &char) -> bool {
  matches!(
    *c,
    '+' | '-' | '*' | '/' | '^' | '&' | '|' | '!' | ':' | '=' | '<' | '>' | '~' | '\'' | '.' | '?'
  )
}

// lexer関数
#[allow(unused_assignments)]
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
  let mut tokens = Vec::new();
  let input: Vec<char> = input.chars().collect();

  // 位置情報
  let mut pos = 0;
  // posを更新するマクロ
  macro_rules! lex_a_token {
    ($lexer:expr) => {{
      let (tok, p) = $lexer?;
      tokens.push(tok);
      pos = p;
    }};
  }
  // tokenとposのリストを更新するマクロ
  macro_rules! lex_token_list {
    ($tokenlst:expr) => {{
      let (lst, p) = $tokenlst?;
      let mut mut_lst = lst;
      tokens.append(&mut mut_lst);
      pos = p;
    }};
  }

  lex_token_list!(lex_program(&input, pos));
  lex_a_token!(lex_eof(pos));
  Ok(tokens)
}

// デフォルト
fn lex_program(input: &Vec<char>, pos: usize) -> Result<(Vec<Token>, usize), LexError> {
  let mut tokens = Vec::new();
  let mut pos = pos;

  // posを更新するマクロ
  macro_rules! lex_a_token {
    ($lexer:expr) => {{
      let (tok, p) = $lexer?;
      tokens.push(tok);
      pos = p;
    }};
  }

  while pos < input.len() {
    match input[pos] {
      // binop_divides
      '%' => {
        let ((), p) = lex_comment(input, pos + 1);
        pos = p;
      }
      '(' => {
        lex_a_token!(lex_lparen(pos));
      }
      ')' => {
        lex_a_token!(lex_rparen(pos));
      }
      '{' => {
        lex_a_token!(lex_lbracket(pos));
      }
      '}' => {
        lex_a_token!(lex_rbracket(pos));
      }
      ';' => {
        lex_a_token!(lex_semicolon(pos));
      }

      // アルファベットと数字と'_'が続く限りvarへ
      '_' => lex_a_token!(lex_identifier(input, pos)),

      ':' => {
        lex_a_token!(lex_colon(pos));
      }
      ',' => {
        lex_a_token!(lex_comma(pos));
      }

      // 次の文字がopsymbolならbinop_eq
      // そうでないならdef_eq
      '=' => match input.get(pos + 1) {
        Some(v) => {
          if is_opsymbol(v) {
            lex_a_token!(lex_binop_eq(input, pos))
          } else {
            lex_a_token!(lex_def_eq(pos))
          }
        }
        None => lex_a_token!(lex_def_eq(pos)),
      },

      '*' => {
        lex_a_token!(lex_binop_times(input, pos));
      }

      '/' => {
        lex_a_token!(lex_binop_divides(input, pos));
      }

      '+' => {
        lex_a_token!(lex_binop_plus(input, pos));
      }

      // 次の文字が数字か'.'ならintかfloat
      // それ以外ならbinop_minus
      '-' => match input.get(pos + 1) {
        Some(v) => {
          if is_digit(v) || &'.' == v {
            lex_a_token!(lex_int_or_float(input, pos))
          } else {
            lex_a_token!(lex_binop_minus(input, pos))
          }
        }
        None => lex_a_token!(lex_binop_minus(input, pos)),
      },

      '^' => {
        lex_a_token!(lex_binop_hat(input, pos));
      }

      '&' => {
        lex_a_token!(lex_binop_amp(input, pos));
      }

      '|' => {
        lex_a_token!(lex_binop_bar(input, pos));
      }

      '<' => {
        lex_a_token!(lex_binop_gt(input, pos));
      }

      '>' => {
        lex_a_token!(lex_binop_lt(input, pos));
      }

      // identifier = (small (digit | latin | "_")*)
      // 登録してあった文字列以外はvarに
      c if is_small(&c) => lex_a_token!(lex_identifier(input, pos)),

      // constructor = (capital (digit | latin | "_")*)
      c if is_capital(&c) => lex_a_token!(lex_constructor(input, pos)),

      c if is_digit(&c) => lex_a_token!(lex_int_or_float(input, pos)),
      '.' => lex_a_token!(lex_int_or_float(input, pos)),

      ' ' | '\n' | '\t' => {
        let ((), p) = skip_spaces(input, pos)?;
        pos = p;
      }
      c => return Err(error_invalid_char(c, types::Range::make(pos, 1))),
    }
  }
  Ok((tokens, pos))
}

// 次の文字もそのトークンだったらposを1つ移動させる関数
fn recognize_many(input: &Vec<char>, mut pos: usize, mut f: impl FnMut(char) -> bool) -> usize {
  while pos < input.len() && f(input[pos]) {
    pos += 1;
  }
  pos
}

// 空白等を無視する
fn skip_spaces(input: &Vec<char>, pos: usize) -> Result<((), usize), LexError> {
  let pos = recognize_many(input, pos, |c| ' ' == c || '\t' == c || '\n' == c);
  Ok(((), pos))
}

// 改行文字が来るまで読み飛ばす
fn lex_comment(input: &Vec<char>, mut pos: usize) -> ((), usize) {
  while pos < input.len() && ('\n' != input[pos]) {
    pos += 1;
  }
  ((), pos)
}

//identifier = (small (digit | latin | "_")*)
//続くところまで取得
fn lex_identifier(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end_pos = recognize_many(input, pos + 1, |c| {
    is_small(&c) || is_capital(&c) || is_digit(&c) || c == '_'
  });
  let v_string: String = input[start..end_pos].iter().collect();
  let v_str = v_string.as_str();
  match v_str {
    "true" => Ok((
      (
        TokenKind::TRUE,
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
    "false" => Ok((
      (
        TokenKind::FALSE,
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
    "if" => Ok((
      (TokenKind::IF, types::Range::make_start_end(start, end_pos)),
      end_pos,
    )),
    "then" => Ok((
      (
        TokenKind::THEN,
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
    "else" => Ok((
      (
        TokenKind::ELSE,
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
    "fn" => Ok((
      (TokenKind::FN, types::Range::make_start_end(start, end_pos)),
      end_pos,
    )),
    "let" => Ok((
      (TokenKind::LET, types::Range::make_start_end(start, end_pos)),
      end_pos,
    )),
    "mut" => Ok((
      (TokenKind::MUT, types::Range::make_start_end(start, end_pos)),
      end_pos,
    )),
    "while" => Ok((
      (
        TokenKind::WHILE,
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
    _ => Ok((
      (
        TokenKind::VAR(v_string),
        types::Range::make_start_end(start, end_pos),
      ),
      end_pos,
    )),
  }
}

// constructor = (capital (digit | latin | "_")*)
// constructorが続くところまで取得
fn lex_constructor(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end_pos = recognize_many(input, pos + 1, |c| {
    is_small(&c) || is_capital(&c) || is_digit(&c) || c == '_'
  });
  let v_string: String = input[start..end_pos].iter().collect();
  Ok((
    (
      TokenKind::CONSTRUCTOR(v_string),
      types::Range::make_start_end(start, end_pos),
    ),
    end_pos,
  ))
}

//EOF,
//DEF_EQ
//LPAREN,
//RPAREN,
//SEMICOLON,
//COLON,
//COMMA,

fn lex_eof(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::EOF, types::Range::make(pos, 1)), pos + 1))
}

fn lex_def_eq(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::DEF_EQ, types::Range::make(pos, 1)), pos + 1))
}

fn lex_lparen(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::LPAREN, types::Range::make(pos, 1)), pos + 1))
}

fn lex_rparen(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::RPAREN, types::Range::make(pos, 1)), pos + 1))
}

fn lex_lbracket(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::LBRACKET, types::Range::make(pos, 1)), pos + 1))
}

fn lex_rbracket(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::RBRACKET, types::Range::make(pos, 1)), pos + 1))
}

fn lex_semicolon(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::SEMICOLON, types::Range::make(pos, 1)), pos + 1))
}

fn lex_colon(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::COLON, types::Range::make(pos, 1)), pos + 1))
}

fn lex_comma(pos: usize) -> Result<(Token, usize), LexError> {
  Ok(((TokenKind::COMMA, types::Range::make(pos, 1)), pos + 1))
}

// `*` から始まる記号群
// 1つだけでも良い
fn lex_binop_times(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `/` から始まる記号群
// 1つだけでも良い
fn lex_binop_divides(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `+` から始まる記号群
// 1つだけでも良い
fn lex_binop_plus(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `-` から始まる記号群
// 1つだけでも良い
fn lex_binop_minus(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `^` から始まる記号群
// 1つだけでも良い
fn lex_binop_hat(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `&` から始まる記号群
// 1つだけでも良い
fn lex_binop_amp(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `|` から始まる記号群
// 1つだけでも良い
fn lex_binop_bar(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `<` から始まる記号群
// 1つだけでも良い
fn lex_binop_gt(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `>` から始まる記号群
// 1つだけでも良い
fn lex_binop_lt(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// `=` から始まる記号群
// 1つだけでも良い
fn lex_binop_eq(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let end = recognize_many(input, start, |c| is_opsymbol(&c));
  let bin_string: String = input[start..end].iter().collect();
  Ok((
    (
      TokenKind::BINOP_TIMES(bin_string),
      types::Range::make_start_end(start, end),
    ),
    end,
  ))
}

// intかfloatに解析
fn lex_int_or_float(input: &Vec<char>, pos: usize) -> Result<(Token, usize), LexError> {
  let start = pos;
  let start_n = match input.get(pos) {
    None => pos,
    Some('-') => pos + 1,
    _ => pos,
  };
  let int_end = recognize_many(input, start_n, |c| is_digit(&c));
  let is_float = match input.get(int_end) {
    None => false,
    Some('.') => true,
    _ => false,
  };
  let end = if is_float {
    recognize_many(input, int_end + 1, |c| is_digit(&c))
  } else {
    int_end
  };
  let range = types::Range::make_start_end(start, end);
  let tok_string: String = input[start..end].iter().collect();
  let tok = if is_float {
    let fl_res = tok_string.parse();
    match fl_res {
      Ok(f) => TokenKind::FLOATCONST(f),
      Err(_) => return Err(error_undefined_token(tok_string, range)),
    }
  } else {
    let int_res = tok_string.parse();
    match int_res {
      Ok(i) => TokenKind::INTCONST(i),
      Err(_) => return Err(error_undefined_token(tok_string, range)),
    }
  };
  Ok(((tok, range), end))
}
