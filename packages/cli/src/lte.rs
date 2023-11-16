// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// why was this module named `lte`? short for "Light Template Engine" and might be extracted into its own crate.

// TODOs:
// 1. negative boolean
// 2. multiple condition in the same if
// 3. equality checks
// 4. infinte loop when missing a closing `%}`

use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct TemplateParseError {
    message: String,
}
impl TemplateParseError {
    fn new(message: String) -> Self {
        Self { message }
    }
}
impl std::fmt::Display for TemplateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse template: {}", self.message)
    }
}
impl std::error::Error for TemplateParseError {}
type Result<T> = std::result::Result<T, TemplateParseError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Token<'a> {
    OBracket,
    If,
    Var(&'a str),
    Else,
    EndIf,
    CBracket,
    Text(&'a str),
    Invalid(usize),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::OBracket => write!(f, "{{%"),
            Token::If => write!(f, "if"),
            Token::Var(var) => write!(f, "{} (variable)", var),
            Token::Else => write!(f, "else"),
            Token::EndIf => write!(f, "endif"),
            Token::CBracket => write!(f, "%}}"),
            Token::Text(_) => write!(f, "(text)"),
            Token::Invalid(col) => write!(f, "invalid token at {col}"),
        }
    }
}

const KEYWORDS: &[(&str, Token)] = &[
    ("if", Token::If),
    ("else", Token::Else),
    ("endif", Token::EndIf),
];

struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    len: usize,
    cursor: usize,
    in_bracket: bool,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        let len = bytes.len();
        Self {
            len,
            bytes,
            input,
            cursor: 0,
            in_bracket: false,
        }
    }

    fn current_char(&self) -> char {
        self.bytes[self.cursor] as char
    }

    fn next_char(&self) -> char {
        self.bytes[self.cursor + 1] as char
    }

    fn skip_whitespace(&mut self) {
        while self.cursor < self.len && self.current_char().is_whitespace() {
            self.cursor += 1;
        }
    }

    fn is_symbol_start(&self) -> bool {
        let c = self.current_char();
        c.is_alphabetic() || c == '_'
    }

    fn is_symbol(&self) -> bool {
        let c = self.current_char();
        c.is_alphanumeric() || c == '_'
    }

    fn read_symbol(&mut self) -> &'a str {
        let start = self.cursor;
        while self.is_symbol() {
            self.cursor += 1;
        }
        let end = self.cursor - 1;
        &self.input[start..=end]
    }

    fn next(&mut self) -> Option<Token<'a>> {
        if self.in_bracket {
            self.skip_whitespace();
        }

        if self.cursor >= self.len {
            return None;
        }

        if self.current_char() == '{' && self.next_char() == '%' {
            self.in_bracket = true;
            self.cursor += 2;
            return Some(Token::OBracket);
        }

        if self.current_char() == '%' && self.next_char() == '}' {
            self.in_bracket = false;
            self.cursor += 2;
            return Some(Token::CBracket);
        }

        if self.in_bracket {
            if self.is_symbol_start() {
                let symbol = self.read_symbol();
                for (keyword, t) in KEYWORDS {
                    if *keyword == symbol {
                        return Some(*t);
                    }
                }

                return Some(Token::Var(symbol));
            } else {
                self.cursor += 1;
                return Some(Token::Invalid(self.cursor));
            }
        }

        if !self.in_bracket {
            let start = self.cursor;
            while !(self.current_char() == '{' && self.next_char() == '%') {
                self.cursor += 1;

                if self.cursor >= self.len {
                    break;
                }
            }
            let end = self.cursor - 1;
            return Some(Token::Text(&self.input[start..=end]));
        }

        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Stmt<'a> {
    Text(&'a str),
    Var(&'a str),
    If {
        var: &'a str,
        condition: Vec<Stmt<'a>>,
        else_condition: Option<Vec<Stmt<'a>>>,
    },
}

impl<'a> Stmt<'a> {
    fn execute<T: ToString>(&self, out: &mut String, data: &HashMap<&str, T>) -> Result<()> {
        match self {
            Stmt::Text(t) => out.push_str(t),
            Stmt::Var(v) => {
                let value = data.get(v).ok_or_else(|| {
                    TemplateParseError::new(format!("Unrecognized variable: {v}"))
                })?;
                out.push_str(&value.to_string())
            }
            Stmt::If {
                var,
                condition,
                else_condition,
            } => {
                let value = data
                    .get(var)
                    .ok_or_else(|| {
                        TemplateParseError::new(format!("Unrecognized variable: {var}"))
                    })?
                    .to_string();

                let evaluated = if value == "true" {
                    condition.as_slice()
                } else if let Some(else_condition) = else_condition {
                    else_condition.as_slice()
                } else {
                    &[]
                };

                for stmt in evaluated {
                    stmt.execute(out, data)?;
                }
            }
        }

        Ok(())
    }
}

struct Parser<'a> {
    tokens: &'a [Token<'a>],
    len: usize,
    cursor: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            len: tokens.len(),
            tokens,
            cursor: 0,
        }
    }

    fn current_token(&self) -> Token<'a> {
        self.tokens[self.cursor]
    }

    fn skip_brackets(&mut self) {
        if self.cursor < self.len {
            while self.current_token() == Token::OBracket || self.current_token() == Token::CBracket
            {
                self.cursor += 1;

                if self.cursor >= self.len {
                    break;
                }
            }
        }
    }

    fn consume_text(&mut self) -> Option<&'a str> {
        if let Token::Text(text) = self.current_token() {
            self.cursor += 1;
            Some(text)
        } else {
            None
        }
    }

    fn consume_var(&mut self) -> Option<&'a str> {
        if let Token::Var(var) = self.current_token() {
            self.cursor += 1;
            Some(var)
        } else {
            None
        }
    }

    fn consume_if(&mut self) -> Result<Option<Stmt<'a>>> {
        if self.current_token() == Token::If {
            self.cursor += 1;

            let var = self.consume_var().ok_or_else(|| {
                TemplateParseError::new(format!(
                    "expected variable after if, found: {}",
                    self.current_token()
                ))
            })?;

            let mut condition = Vec::new();
            while self.current_token() != Token::Else || self.current_token() != Token::EndIf {
                match self.next()? {
                    Some(stmt) => condition.push(stmt),
                    None => break,
                }
            }

            let else_condition = if self.current_token() == Token::Else {
                self.cursor += 1;

                let mut else_condition = Vec::new();
                while self.current_token() != Token::EndIf {
                    match self.next()? {
                        Some(stmt) => else_condition.push(stmt),
                        None => break,
                    }
                }

                Some(else_condition)
            } else {
                None
            };

            if self.current_token() == Token::EndIf {
                self.cursor += 1;
            } else {
                return Err(TemplateParseError::new(format!(
                    "expected endif, found: {}",
                    self.current_token()
                )));
            }

            Ok(Some(Stmt::If {
                var,
                condition,
                else_condition,
            }))
        } else {
            Ok(None)
        }
    }

    fn next(&mut self) -> Result<Option<Stmt<'a>>> {
        self.skip_brackets();

        if self.cursor >= self.len {
            return Ok(None);
        }

        if let t @ Token::Invalid(_) = self.current_token() {
            return Err(TemplateParseError {
                message: t.to_string(),
            });
        }

        let text = self.consume_text();
        if text.is_some() {
            return Ok(text.map(Stmt::Text));
        }

        let var = self.consume_var();
        if var.is_some() {
            return Ok(var.map(Stmt::Var));
        }

        let if_ = self.consume_if()?;
        if if_.is_some() {
            return Ok(if_);
        }

        Ok(None)
    }
}

pub fn render<T: ToString>(template: &str, data: &HashMap<&str, T>) -> Result<String> {
    let tokens: Vec<Token> = Lexer::new(template).collect();
    let mut parser = Parser::new(&tokens);
    let mut stmts: Vec<Stmt> = Vec::new();
    while let Some(stmt) = parser.next()? {
        stmts.push(stmt);
    }
    let mut out = String::new();
    for stmt in stmts {
        stmt.execute(&mut out, data)?;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn it_replaces_variable() {
        let template = "<html>Hello {% name %}</html>";
        let data: HashMap<&str, &str> = [("name", "world")].into();
        let rendered = render(template, &data).expect("it should render");
        assert_eq!(rendered, "<html>Hello world</html>")
    }

    #[test]
    fn it_replaces_variable_with_new_lines() {
        let template = r#"
        <html>
        <h1>Hello<h2>
        <em>{% name %}</em>
        </html>"#;
        let data: HashMap<&str, &str> = [("name", "world")].into();
        let rendered = render(template, &data).expect("it should render");
        let expected = r#"
        <html>
        <h1>Hello<h2>
        <em>world</em>
        </html>"#;
        assert_eq!(rendered, expected)
    }

    #[test]
    fn it_performs_condition() {
        let template = "<html>Hello {% if alpha %}alpha{% else %}stable{% endif %}</html>";
        let data: HashMap<&str, bool> = [("alpha", true)].into();
        let rendered = render(template, &data).expect("it should render");
        assert_eq!(rendered, "<html>Hello alpha</html>")
    }

    #[test]
    fn it_performs_else_condition() {
        let template = "<html>Hello {% if alpha %}alpha{% else %}stable{% endif %}</html>";
        let data: HashMap<&str, bool> = [("alpha", false)].into();
        let rendered = render(template, &data).expect("it should render");
        assert_eq!(rendered, "<html>Hello stable</html>")
    }

    #[test]
    fn it_performs_condition_with_new_lines() {
        let template = r#"
        <html>
        <h1>Hello<h2>{% if alpha %}
        <em>alpha</em>{% else %}
        <em>stable</em>{% endif %}
        </html>"#;
        let data: HashMap<&str, bool> = [("alpha", true)].into();
        let rendered = render(template, &data).expect("it should render");
        let expected = r#"
        <html>
        <h1>Hello<h2>
        <em>alpha</em>
        </html>"#;
        assert_eq!(rendered, expected)
    }

    #[test]
    fn it_replaces_variable_within_if() {
        let template = r#"
        <html>
        <h1>Hello<h2>{% if alpha %}
        <em>{% alpha_str %}</em>{% else %}
        <em>stable</em>{% endif %}
        </html>"#;
        let data: HashMap<&str, &str> = [("alpha", "true"), ("alpha_str", "holla alpha")].into();
        let rendered = render(template, &data).expect("it should render");
        let expected = r#"
        <html>
        <h1>Hello<h2>
        <em>holla alpha</em>
        </html>"#;
        assert_eq!(rendered, expected)
    }

    #[test]
    fn it_performs_nested_conditions() {
        let template = r#"
        <html>
        <h1>Hello<h2>{% if alpha %}
        <em>{% alpha_str %}</em>{% else %}
        <em>{% if beta %}beta{%else%}stable{%endif%}</em>{% endif %}
        </html>"#;
        let data: HashMap<&str, &str> = [
            ("alpha", "false"),
            ("beta", "true"),
            ("alpha_str", "holla alpha"),
        ]
        .into();
        let rendered = render(template, &data).expect("it should render");
        let expected = r#"
        <html>
        <h1>Hello<h2>
        <em>beta</em>
        </html>"#;
        assert_eq!(rendered, expected)
    }

    #[test]
    fn it_panics() {
        let template = "<html>Hello {% name }</html>";
        let data: HashMap<&str, &str> = [("name", "world")].into();
        let rendered = render(template, &data);
        assert!(rendered.is_err())
    }
}
