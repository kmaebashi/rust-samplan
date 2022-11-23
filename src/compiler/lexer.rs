use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;
use std::process::exit;
use std::fmt;

enum Status {
    Initial,
    Comment,
    IntPart,
    DecimalPoint,
    AfterDecimalPoint,
    AlNum,
    String,
    Operator
}

#[derive(Clone, PartialEq)]
pub enum TokenType {
    Var,
    Boolean,
    Int,
    Real,
    String,
    Function,
    True,
    False,
    If,
    Elsif,
    Else,
    While,
    Return,
    Identifier(String),
    IntValue(i32),
    RealValue(f64),
    StringValue(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Equal,
    NotEqual,
    Assignment,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual,
    LogicalAnd,
    LogicalOr,
    Increment,
    Decrement,
    Comma,
    Semicolon,
    EndOfFile
}

impl fmt::Display for TokenType {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Var => write!(f, "var"),
            TokenType::Boolean => write!(f, "boolean"),
            TokenType::Int => write!(f, "int"),
            TokenType::Real => write!(f, "real"),
            TokenType::String => write!(f, "string"),
            TokenType::Function => write!(f, "function"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::If => write!(f, "if"),
            TokenType::Elsif => write!(f, "elsif"),
            TokenType::Else => write!(f, "else"),
            TokenType::While => write!(f, "while"),
            TokenType::Return => write!(f, "return"),
            TokenType::Identifier(name) => write!(f, "Identifier({})", name),
            TokenType::IntValue(value) => write!(f, "IntValue({})", value),
            TokenType::RealValue(value) => write!(f, "RealValue({})", value),
            TokenType::StringValue(str) => write!(f, "String({})", str),
            TokenType::Plus => write!(f, "+"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Equal => write!(f, "="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Assignment => write!(f, ":="),
            TokenType::GreaterThan => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::LessThan => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::LogicalAnd => write!(f, "&&"),
            TokenType::LogicalOr => write!(f, "||"),
            TokenType::Increment => write!(f, "++"),
            TokenType::Decrement => write!(f, "--"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::EndOfFile => write!(f, "EOF")
        }
    }
}

const KEYWORD_ARRAY:&[(&str, TokenType)] = &[
    ("var", TokenType::Var),
    ("boolean", TokenType::Boolean),
    ("int", TokenType::Int),
    ("real", TokenType::Real),
    ("string", TokenType::String),
    ("function", TokenType::Function),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("elsif", TokenType::Elsif),
    ("else", TokenType::Else),
    ("while", TokenType::While),
    ("return", TokenType::Return)
];

const OPERATOR_ARRAY:&[(&str, TokenType)] = &[
    ("+", TokenType::Plus),
    ("-", TokenType::Minus),
    ("*", TokenType::Asterisk),
    ("/", TokenType::Slash),
    ("{", TokenType::LeftBrace),
    ("}", TokenType::RightBrace),
    ("(", TokenType::LeftParen),
    (")", TokenType::RightParen),
    ("=", TokenType::Equal),
    ("!=", TokenType::NotEqual),
    (":=", TokenType::Assignment),
    (">", TokenType::GreaterThan),
    (">=", TokenType::GreaterEqual),
    ("<", TokenType::LessThan),
    ("<=", TokenType::LessEqual),
    ("&&", TokenType::LogicalAnd),
    ("||", TokenType::LogicalOr),
    ("++", TokenType::Increment),
    ("--", TokenType::Decrement),
    (",", TokenType::Comma),
    (";", TokenType::Semicolon),
];

pub struct Token {
    pub token_type: TokenType,
    pub line_number: i32
}

impl Token {
    fn new(token_type: TokenType, line_number: i32) -> Token {
        Token {
            token_type,
            line_number: line_number
        }
    }
}

pub struct Lexer {
    current_line_number: i32,
    current_line: Vec<char>,
    current_index: usize,
    lines: Lines<BufReader<File>>,
    at_eol: bool
}

impl Lexer {
    pub fn new(src_path :&str) -> Lexer {
        let file = File::open(src_path).expect("ファイルが見つかりません。");
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let first_line = match lines.next() {
            Some(line) => line,
            None => {
                println!("ファイルが空です。");
                exit(1);
            }
        }.expect("ファイルが読み込めません。");
        Lexer {
            current_line_number: 1,
            current_line: first_line.chars().collect(),
            current_index: 0,
            lines: lines,
            at_eol: false
        }
    }

    pub fn get_token(&mut self) -> Token {
        let mut current_status = Status::Initial;
        let mut start_index = 0;
        let token_type: TokenType;

        loop {
            let opt_ch: Option<char> = self.getc();
            let ch = match opt_ch {
                None => {
                    let token = Token::new(TokenType::EndOfFile, self.current_line_number);
                    return token;
                }
                Some(ch) => ch
            };
            current_status = match current_status {
                Status::Initial => {
                    match ch {
                        '#' => {
                            Status::Comment
                        },
                        '0'..='9' => {
                            start_index = self.current_index - 1;
                            Status::IntPart
                        },
                        'a'..='z' | 'A'..='Z' | '_' => {
                            start_index = self.current_index - 1;
                            Status::AlNum
                        },
                        '\"' => {
                            start_index = self.current_index;
                            Status::String
                        },
                        ' ' | '\t' | '\r' | '\n' => {
                            if ch == '\n' {
                                self.current_line_number += 1;
                            }
                            Status::Initial
                        },
                        _ => {
                            if Lexer::is_operator_start(ch) {
                                start_index = self.current_index - 1;
                                Status::Operator
                            } else {
                                println!("不正な文字です({})", ch);
                                exit(1);
                            }
                        }
                    } 
                },
                Status::Comment => {
                    if ch == '\n' {
                        self.current_line_number += 1;
                        Status::Initial
                    } else {
                        Status::Comment
                    }
                },
                Status::IntPart => {
                    match ch {
                        '0'..='9' => Status::IntPart,
                        '.' => Status::DecimalPoint,
                        _ => {
                            self.ungetc();
                            let vec_slice = &self.current_line[start_index..self.current_index];
                            let str: String = vec_slice.iter().collect::<String>();
                            let int_value: i32 = str.parse().expect("Not a number!");
                            token_type = TokenType::IntValue(int_value);
                            break;
                        }
                    }
                },
                Status::DecimalPoint => {
                    match ch {
                        '0'..='9' => Status::AfterDecimalPoint,
                        _ => {
                            println!("不正な文字です({})", ch);
                            exit(1);                            
                        }
                    }
                }
                Status::AfterDecimalPoint => {
                    match ch {
                        '0'..='9' => Status::AfterDecimalPoint,
                        _ => {
                            self.ungetc();
                            let vec_slice = &self.current_line[start_index..self.current_index];
                            let str: String = vec_slice.iter().collect::<String>();
                            let real_value: f64 = str.parse().expect("Not a number!");
                            token_type = TokenType::RealValue(real_value);
                            break;
                        }
                    }
                },
                Status::AlNum => {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => Status::AlNum,
                        _ => {
                            self.ungetc();
                            let vec_slice = &self.current_line[start_index..self.current_index];
                            token_type = match Self::is_keyword(&vec_slice) {
                                Some(keyword) => keyword,
                                None => TokenType::Identifier(vec_slice.iter().collect::<String>())
                            };
                            break;
                        }
                    }
                },
                Status::String => {
                    match ch {
                        '\"' => {
                            let vec_slice = &self.current_line[start_index..(self.current_index - 1)];
                            let str = Self::convert_string_escape(&vec_slice);
                            token_type = TokenType::StringValue(str);
                            break;
                        }
                        '\n' => {
                            println!("文字列の中に改行は入れられません。");
                            exit(1);                              
                        }
                        _ => Status::String
                    }
                },
                Status::Operator => {
                    let vec_slice = &self.current_line[start_index..self.current_index];
                    if Lexer::in_operator(&vec_slice) && !self.at_eol {
                        Status::Operator
                    } else {
                        self.ungetc();
                        let vec_slice = &self.current_line[start_index..self.current_index];
                        token_type = Lexer::get_operator(&vec_slice);
                        break;
                    }
                }
            }
        }
        Token::new(token_type, self.current_line_number)
    }

    fn getc(&mut self) -> Option<char> {
        if self.at_eol {
            self.current_line = match self.lines.next() {
                Some(line) => line,
                None => {
                    return None;
                }
            }.expect("ファイルが読み込めません。").chars().collect();
            self.current_index = 0;
            self.at_eol = false;
        }
        if self.current_index < self.current_line.len() {
            let ret = self.current_line[self.current_index];            
            self.current_index += 1;
            Some(ret)
        } else {
            assert_eq!(self.current_index, self.current_line.len());
            self.at_eol = true;
            Some('\n')
        }
    }

    fn ungetc(&mut self) {
        if self.current_index > 0 {
            if !self.at_eol {
                self.current_index -= 1;
            } 
        } else {
            panic!("invalid ungetc()");
        }
    }

    fn is_keyword(char_array: &[char]) -> Option<TokenType> {
        let str = char_array.iter().collect::<String>();
        for op in KEYWORD_ARRAY {
            if op.0 == str {
                return Some(op.1.clone());
            }
        }
        return Option::None;
    }

    fn is_operator_start(ch: char) -> bool {
        for op in OPERATOR_ARRAY {
            if op.0.starts_with(ch) {
                return true;
            }
        }
        false
    }

    fn in_operator(char_array: &[char]) -> bool {
        let str = char_array.iter().collect::<String>();
        for op in OPERATOR_ARRAY {
            if op.0.starts_with(&str) {
                return true;
            }
        }
        false
    }

    fn get_operator(char_array: &[char]) -> TokenType {
        let str = char_array.iter().collect::<String>();
        for op in OPERATOR_ARRAY {
            if op.0 == str {
                return op.1.clone();
            }
        }
        panic!("not operator");
    }

    fn convert_string_escape(src: &[char]) -> String {
        let mut dest: String = String::from("");

        let mut escaping = false;
        for ch in src {
            if !escaping {
                if *ch == '\\' {
                    escaping = true;
                } else {
                    dest.push(*ch);
                }
            } else {
                if *ch == 'n' {
                    dest.push('\n');
                    escaping = false;
                } else {
                    println!("不正なエスケープ文字です({})。", *ch);
                    exit(1);
                }
            }
        }
        if escaping {
            println!("文字列が終端していません。");
            exit(1);
        }
        dest
    }
}

#[test]
fn is_keyword_test() {
    let token_type: Option<TokenType> = Lexer::is_keyword(&['v', 'a', 'r']);
    assert!(token_type.unwrap() == TokenType::Var);
    let token_type: Option<TokenType> = Lexer::is_keyword(&['h', 'o', 'g', 'e']);
    assert!(token_type.is_none());
}

#[test]
fn is_operator_start_test() {
    let result = Lexer::is_operator_start('=');
    assert!(result);

    let result = Lexer::is_operator_start('a');
    assert!(!result);
}