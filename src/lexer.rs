use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str;
use std::collections::VecDeque;
use std::path;
use std::process;
use std::collections::{HashMap, HashSet};

use token::{Symbol, Token};

use ansi_term::{Colour, Style};

pub struct Lexer {
    source: String,
    pos: usize,
}

impl Lexer {
    pub fn new(source_file_name: &str) -> Lexer {
        let mut file = OpenOptions::new()
            .read(true)
            .open(source_file_name.to_string())
            .unwrap_or_else(|_| {
                println!(
                    "{} not found such file '{}'",
                    Colour::Red.bold().paint("error:"),
                    Style::new().underline().paint(source_file_name)
                );
                ::std::process::exit(0);
            });

        let mut file_body = String::new();
        file.read_to_string(&mut file_body).unwrap_or_else(|e| {
            println!(
                "an error occurred while reading file '{}'\n{} {}",
                Style::new().underline().paint(source_file_name),
                Colour::Red.bold().paint("error:"),
                e
            );
            ::std::process::exit(-1)
        });

        Lexer {
            source: file_body,
            pos: 0,
        }
    }
}

impl Lexer {
    pub fn read_token(&mut self) -> Result<Token, ()> {
        match self.next_char()? {
            'a'...'z' | 'A'...'Z' | '_' => self.read_identifier(),
            '0'...'9' => self.read_number(),
            '\"' => self.read_string_literal(),
            '\n' => self.read_newline(),
            c if c.is_whitespace() => {
                self.skip_whitespace()?;
                self.read_token()
            }
            _ => self.read_symbol(),
        }
    }
}

impl Lexer {
    pub fn read_identifier(&mut self) -> Result<Token, ()> {
        let pos = self.pos;
        let ident = self.skip_while(|c| c.is_alphanumeric() || c == '_')?;
        Ok(Token::new_identifier(ident, pos))
    }
}

impl Lexer {
    pub fn read_number(&mut self) -> Result<Token, ()> {
        let pos = self.pos;
        let mut is_float = false;
        let mut last = self.next_char()?;
        let mut num = self.skip_while(|c| {
            is_float = is_float || c == '.';
            let is_f = "eEpP".contains(last) && "+-".contains(c);
            if !c.is_alphanumeric() && c != '.' && !is_f {
                is_float = is_float || is_f;
                false
            } else {
                last = c;
                true
            }
        })?;
        if is_float {
            // Ignores suffix
            let f: f64 = num.trim_right_matches(|c| match c {
                'a'...'z' | 'A'...'Z' | '+' | '-' => true,
                _ => false,
            }).parse()
                .unwrap();
            Ok(Token::new_float(f, pos))
        } else {
            // TODO: suffix supporting
            let i = if num.len() > 2 && num.chars().nth(1).unwrap() == 'x' {
                self.read_hex_num(&num[2..])
            } else if num.len() > 2 && num.chars().nth(1).unwrap() == 'b' {
                self.read_hex_num(&num[2..])
            } else if num.chars().nth(0).unwrap() == '0' {
                self.read_oct_num(&num[1..])
            } else {
                self.read_dec_num(num.as_str())
            };
            Ok(Token::new_int(i, pos))
        }
    }

    fn read_hex_num(&mut self, num_literal: &str) -> i64 {
        num_literal.chars().fold(0, |n, c| match c {
            '0'...'9' | 'A'...'F' | 'a'...'f' => n * 16 + c.to_digit(16).unwrap() as i64,
            _ => n,
        })
    }

    fn read_dec_num(&mut self, num_literal: &str) -> i64 {
        num_literal.chars().fold(0, |n, c| match c {
            '0'...'9' => n * 10 + c.to_digit(10).unwrap() as i64,
            _ => n,
        })
    }

    fn read_oct_num(&mut self, num_literal: &str) -> i64 {
        num_literal.chars().fold(0, |n, c| match c {
            '0'...'7' => n * 8 + c.to_digit(8).unwrap() as i64,
            _ => n,
        })
    }
    fn read_bin_num(&mut self, num_literal: &str) -> i64 {
        num_literal.chars().fold(0, |n, c| match c {
            '0' | '1' => n * 2 + c.to_digit(2).unwrap() as i64,
            _ => n,
        })
    }
}

impl Lexer {
    pub fn read_string_literal(&mut self) -> Result<Token, ()> {
        let pos = self.pos;
        assert_eq!(self.skip_char()?, '\"');
        // TODO: support escape sequence
        let s = self.skip_while(|c| c != '\"')?;
        assert_eq!(self.skip_char()?, '\"');
        Ok(Token::new_string(s, pos))
    }
}

impl Lexer {
    pub fn read_newline(&mut self) -> Result<Token, ()> {
        assert_eq!(self.skip_char()?, '\n');
        Ok(Token::new_newline(self.pos))
    }
}

impl Lexer {
    pub fn read_symbol(&mut self) -> Result<Token, ()> {
        let pos = self.pos;
        let mut symbol = Symbol::Hash;
        let c = self.skip_char()?; // ???
        match c {
            '+' | '-' => match self.next_char()? {
                '=' => {
                    assert_eq!(self.skip_char()?, '=');
                    if c == '+' {
                        symbol = Symbol::AssignAdd;
                    } else if c == '-' {
                        symbol = Symbol::AssignSub;
                    }
                }
                '>' => {
                    assert_eq!(self.skip_char()?, '>');
                    if c == '-' {
                        symbol = Symbol::Arrow;
                    }
                }
                '+' => {
                    assert_eq!(self.skip_char()?, '+');
                    if c == '+' {
                        symbol = Symbol::Inc;
                    }
                }
                '-' => {
                    assert_eq!(self.skip_char()?, '-');
                    if c == '-' {
                        symbol = Symbol::Dec;
                    }
                }
                _ => if c == '+' {
                    symbol = Symbol::Add;
                } else if c == '-' {
                    symbol = Symbol::Sub;
                },
            },
            '*' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::AssignMul
                } else {
                    symbol = Symbol::Asterisk
                }
            }
            '/' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::AssignDiv
                } else {
                    symbol = Symbol::Div
                }
            }
            '%' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::AssignMod
                } else {
                    symbol = Symbol::Mod
                }
            }
            '=' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::Eq
                } else {
                    symbol = Symbol::Assign
                }
            }
            '^' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::AssignXor
                } else {
                    symbol = Symbol::Mod
                }
            }
            '!' => {
                if self.next_char_is('=')? {
                    symbol = Symbol::Ne
                } else {
                    symbol = Symbol::Not
                }
            }
            '<' | '>' | '&' | '|' => {
                let mut single = true;
                if self.skip_char_is(c)? {
                    symbol = match c {
                        '<' => Symbol::Shl,
                        '>' => Symbol::Shr,
                        '&' => Symbol::LAnd,
                        '|' => Symbol::LOr,
                        _ => unreachable!(),
                    };
                    single = false;
                }
                if self.skip_char_is('=')? {
                    symbol = match c {
                        '<' => Symbol::Le,
                        '>' => Symbol::Ge,
                        '&' => Symbol::AssignAnd,
                        '|' => Symbol::AssignOr,
                        _ => unreachable!(),
                    };
                    single = false;
                }
                if single {
                    symbol = match c {
                        '<' => Symbol::Lt,
                        '>' => Symbol::Gt,
                        '&' => Symbol::Ampersand,
                        '|' => Symbol::Or,
                        _ => unreachable!(),
                    };
                }
            }
            '.' => symbol = Symbol::Point,
            _ => {}
        };
        Ok(Token::new_symbol(symbol, pos))
    }
}

impl Lexer {
    fn next_char_is(&self, c: char) -> Result<bool, ()> {
        Ok(self.next_char()? == c)
    }

    fn skip_char_is(&mut self, c: char) -> Result<bool, ()> {
        if self.next_char()? == c {
            self.skip_char()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn skip_whitespace(&mut self) -> Result<(), ()> {
        self.skip_while(char::is_whitespace).and(Ok(()))
    }

    fn skip_while<F>(&mut self, mut f: F) -> Result<String, ()>
    where
        F: FnMut(char) -> bool,
    {
        let mut v = vec![];
        while f(self.next_char()?) {
            v.push(self.skip_char()? as u8);
        }
        Ok(String::from_utf8_lossy(v.as_slice()).to_owned().to_string())
    }

    fn skip_char(&mut self) -> Result<char, ()> {
        let mut iter = self.source[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(())?;
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        Ok(cur_char)
    }

    fn next_char(&self) -> Result<char, ()> {
        self.source[self.pos..].chars().next().ok_or(())
    }
}
