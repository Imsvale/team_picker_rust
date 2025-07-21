// src/evaluator.rs

use std::str::Chars;
use crate::player::Player;
use crate::composition::PositionRequirements;

#[allow(dead_code)]
#[derive(Debug)]
pub enum EvalError {
    InvalidSyntax(String),
    UnknownStat(String),
    UnknownFunction(String),
    DivisionByZero,
    MissingArguments,
}

pub type EvalResult = Result<f64, EvalError>;

pub fn evaluate(player: &Player, expr: &str) -> EvalResult {
    let mut parser = Parser::new(player, expr);
    let result = parser.parse_expression();
    if parser.peek().is_some() {
        Err(EvalError::InvalidSyntax("Unexpected characters at end".into()))
    } else {
        result
    }
}

struct Parser<'a> {
    player: &'a Player,
    chars: Chars<'a>,
    lookahead: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(player: &'a Player, expr: &'a str) -> Self {
        let mut chars = expr.chars();
        let lookahead = chars.next();
        Parser { player, chars, lookahead }
    }

    fn bump(&mut self) {
        self.lookahead = self.chars.next();
    }

    fn peek(&self) -> Option<char> {
        self.lookahead
    }

    fn consume_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.bump();
        }
    }

    fn parse_expression(&mut self) -> EvalResult {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> EvalResult {
        let mut value = self.parse_mul_div()?;

        loop {
            self.consume_whitespace();
            match self.peek() {
                Some('+') => {
                    self.bump();
                    value += self.parse_mul_div()?;
                }
                Some('-') => {
                    self.bump();
                    value -= self.parse_mul_div()?;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_mul_div(&mut self) -> EvalResult {
        let mut value = self.parse_pow()?;

        loop {
            self.consume_whitespace();
            match self.peek() {
                Some('*') => {
                    self.bump();
                    value *= self.parse_pow()?;
                }
                Some('/') => {
                    self.bump();
                    let denom = self.parse_pow()?;
                    if denom == 0.0 {
                        return Err(EvalError::DivisionByZero);
                    }
                    value /= denom;
                }
                _ => break,
            }
        }

        Ok(value)
    }

    fn parse_pow(&mut self) -> EvalResult {
        let mut base = self.parse_atom()?;

        loop {
            self.consume_whitespace();
            if self.peek() == Some('^') {
                self.bump();
                let exponent = self.parse_atom()?;
                base = base.powf(exponent);
            } else {
                break;
            }
        }

        Ok(base)
    }

    fn parse_atom(&mut self) -> EvalResult {
        self.consume_whitespace();

        match self.peek() {
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_ascii_alphabetic() => self.parse_identifier_or_function(),
            Some('(') => {
                self.bump();
                let value = self.parse_expression()?;
                self.consume_whitespace();
                if self.peek() != Some(')') {
                    return Err(EvalError::InvalidSyntax("Expected ')'".into()));
                }
                self.bump();
                Ok(value)
            }
            Some(c) => Err(EvalError::InvalidSyntax(format!("Unexpected character: '{}'", c))),
            None => Err(EvalError::InvalidSyntax("Unexpected end of input".into())),
        }
    }

    fn parse_number(&mut self) -> EvalResult {
        let mut s = String::new();
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '.') {
            s.push(self.peek().unwrap());
            self.bump();
        }
        s.parse::<f64>().map_err(|_| EvalError::InvalidSyntax(format!("Invalid number: {s}")))
    }

    fn parse_identifier_or_function(&mut self) -> EvalResult {
        let mut name = String::new();
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '_') {
            name.push(self.peek().unwrap());
            self.bump();
        }

        self.consume_whitespace();
        if self.peek() == Some('(') {
            self.bump(); // consume '('
            let args = self.parse_arguments()?;
            self.evaluate_function(&name, args)
        } else {
            self.lookup_stat(&name)
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<f64>, EvalError> {
        let mut args = Vec::new();

        loop {
            self.consume_whitespace();
            if self.peek() == Some(')') {
                self.bump();
                break;
            }

            args.push(self.parse_expression()?);
            self.consume_whitespace();

            match self.peek() {
                Some(',') => {
                    self.bump();
                    continue;
                }
                Some(')') => {
                    self.bump();
                    break;
                }
                Some(c) => {
                    return Err(EvalError::InvalidSyntax(format!("Unexpected character: '{}'", c)))
                }
                None => return Err(EvalError::InvalidSyntax("Unterminated function call".into())),
            }
        }

        Ok(args)
    }

    fn evaluate_function(&self, name: &str, args: Vec<f64>) -> EvalResult {
        match name.to_ascii_uppercase().as_str() {
            "MIN" => args.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or_else(|| EvalError::MissingArguments),
            "MAX" => args.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap())
                .ok_or_else(|| EvalError::MissingArguments),
            "AVERAGE" => {
                if args.is_empty() {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok(args.iter().sum::<f64>() / args.len() as f64)
                }
            }
            "IF" => {
                if args.len() != 3 {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok(if args[0].abs() > 0.5 { args[1] } else { args[2] })
                }
            }
            "POW" => {
                if args.len() != 2 {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok(args[0].powf(args[1]))
                }
            }
            other => Err(EvalError::UnknownFunction(other.to_string())),
        }
    }

    fn lookup_stat(&self, name: &str) -> EvalResult {
        for (key, value) in &self.player.stats {
            if key.eq_ignore_ascii_case(name) {
                return Ok(*value as f64);
            }
        }
        Err(EvalError::UnknownStat(name.to_string()))
    }
}

pub fn evaluate_position(player: &Player, position: &str, reqs: &PositionRequirements) -> EvalResult {
    let expr = reqs
        .position_to_calculation
        .get(position)
        .map(|s| s.as_str())
        .unwrap_or(position); // default to position name if no formula

    evaluate(player, expr)
}

