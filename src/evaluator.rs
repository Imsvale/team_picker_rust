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

type EvalResult = Result<f64, EvalError>;

pub fn evaluate(player: &Player, expr: &str) -> EvalResult {

    // Remove whitespace to greatly simplify parsing!
    let expr = expr.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    let mut parser = Parser::new(player, &expr);

    match parser.parse_expression() {
        Ok(value) => {
            if parser.peek().is_some() {
                Err(EvalError::InvalidSyntax("Unexpected characters at end".into()))
            } else {
                Ok(value)
            }
        }
        Err(e) => Err(e)
    }
}


use std::iter::Peekable;

struct Parser<'a> {
    player: &'a Player,
    chars: Peekable<Chars<'a>>,
}


impl<'a> Parser<'a> {
    fn new(player: &'a Player, expr: &'a str) -> Self {
        let chars = expr.chars().peekable();
        Parser { player, chars }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }
    
    fn bump(&mut self) {
        self.chars.next();
    }
    
    fn consume_symbol(&mut self, sym: &str) -> bool {
        let saved = self.chars.clone();
        let mut matched = true;
    
        for expected in sym.chars() {
            if Some(expected) != self.peek() {
                matched = false;
                break;
            }
            self.bump();
        }
    
        if matched {
            true
        } else {
            self.chars = saved;
            false
        }
    }

    fn consume_comparison_operator(&mut self) -> Result<String, EvalError> {
        let mut op = String::new();
        if let Some(c) = self.peek() {
            op.push(c);
            self.bump();
    
            if let Some('=') = self.peek() {
                op.push('=');
                self.bump();
            }
    
            return Ok(op);
        }
        Err(EvalError::InvalidSyntax("Expected comparison operator".into()))
    }
    

    fn parse_expression(&mut self) -> EvalResult {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> EvalResult {
        let mut value = self.parse_logic_and()?;
    
        loop {
            if self.consume_symbol("||") {
                let rhs = self.parse_logic_and()?;
                value = ((value != 0.0) || (rhs != 0.0)) as i32 as f64;
            } else {
                break;
            }
        }
    
        Ok(value)
    }

    fn parse_logic_and(&mut self) -> EvalResult {
        let mut value = self.parse_comparison()?;
    
        loop {
            if self.consume_symbol("&&") {
                let rhs = self.parse_comparison()?;
                value = ((value != 0.0) && (rhs != 0.0)) as i32 as f64;
            } else {
                break;
            }
        }
    
        Ok(value)
    }
    
    
    fn parse_comparison(&mut self) -> EvalResult {
        let left = self.parse_add_sub()?;  // parse lhs expression
    
        if let Some(op) = self.peek() {
            match op {
                '>' | '<' | '=' | '!' => {
                    let op_str = self.consume_comparison_operator()?;
                    let right = self.parse_add_sub()?;
                    return Ok(match op_str.as_str() {
                        ">"  => (left >  right) as i32 as f64,
                        ">=" => (left >= right) as i32 as f64,
                        "<"  => (left <  right) as i32 as f64,
                        "<=" => (left <= right) as i32 as f64,
                        "==" => (left == right) as i32 as f64,
                        "!=" => (left != right) as i32 as f64,
                        _ => return Err(EvalError::InvalidSyntax(format!("Unknown comparison: {op_str}")))
                    });
                }
                _ => {}
            }
        }
    
        Ok(left)
    }
    
    
    

    fn parse_add_sub(&mut self) -> EvalResult {
        let mut value = self.parse_mul_div()?;

        loop {
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
        
        // Unary minus
        if self.consume_symbol("-") {
            return Ok(-self.parse_atom()?);
        }
        
        // Negation with !
        if self.consume_symbol("!") {
        
            let value = if self.peek() == Some('(') {
                self.bump(); // consume '('
                let v = self.parse_expression()?;

                if self.peek() != Some(')') {
                    return Err(EvalError::InvalidSyntax("Expected ')' after !(...)".to_string()));
                }
                self.bump();
                v
            } else {
                self.parse_atom()?
            };
        
            return Ok((value == 0.0) as i32 as f64);
        }
    
        match self.peek() {
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_ascii_alphabetic() => self.parse_identifier_or_function(),
            Some('(') => {
                self.bump();
                let value = self.parse_expression()?;
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
        
        
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                name.push(c);
                self.bump();
            } else {
                break;
            }
        }

        if name.is_empty() {
            return Err(EvalError::InvalidSyntax("Expected identifier".into()));
        }

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
            if self.peek() == Some(')') {
                self.bump();
                break;
            }

            args.push(self.parse_expression()?);

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
            "NOT" => {
                if args.len() != 1 {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok((args[0] == 0.0) as i32 as f64)
                }
            }
            "AND" => {
                if args.len() != 2 {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok(((args[0] != 0.0) && (args[1] != 0.0)) as i32 as f64)
                }
            }
            "OR" => {
                if args.len() != 2 {
                    Err(EvalError::MissingArguments)
                } else {
                    Ok(((args[0] != 0.0) || (args[1] != 0.0)) as i32 as f64)
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