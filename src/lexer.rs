use std::fs::OpenOptions;
use std::io::prelude::*;
use std::str;
use std::collections::VecDeque;
use std::path;
use std::process;
use std::collections::{HashMap, HashSet};

use token::Token;

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

    pub fn read_token(&mut self) -> Result<Token, ()> {
        match self.next_char()? {
            'a'...'z' | 'A'...'Z' | '_' => self.read_identifier(),
            c if c.is_whitespace() => Err(()),
            _ => Err(()),
        }
    }

    pub fn read_identifier(&mut self) -> Result<Token, ()> {
        let pos = self.pos;
        let mut ident = self.skip_while(|c| c.is_alphanumeric() || c == '_')?;
        Ok(Token::new_identifier(ident, pos))
    }

    fn skip_while<F>(&mut self, f: F) -> Result<String, ()>
    where
        F: Fn(char) -> bool,
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
