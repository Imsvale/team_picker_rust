// src/evaluator.rs

use std::collections::HashMap;
use crate::player::Player;

#[derive(Debug)]
pub enum EvalError {
    InvalidSyntax(String),
    UnknownStat(String),
    DivisionByZero,
    MissingArguments,
    FunctionError(String),
}

pub type EvalResult = Result<f64, EvalError>;

pub fn evaluate(player: &Player, expr: &str) -> EvalResult {
    let expr = expr.trim();

    if expr.is_empty() {
        return Err(EvalError::InvalidSyntax("Empty expression".into()));
    }

    let mut parser = Parser::new(player, expr);
    parser.parse_expression()
}

use std::f64;
use std::str::FromStr;

struct Parser<'a> {
    player: &'a Player,
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(player: &'a Player, input: &'a str) -> Self {
        Parser { player, input }
    }

    fn parse_expression(&self) -> EvalResult {
        self.parse_logic_or(self.input)
    }

    fn parse_logic_or(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["||"], Self::parse_logic_and)
    }

    fn parse_logic_and(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["&&"], Self::parse_comparison)
    }

    fn parse_comparison(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["==", "!=", ">=", "<=", ">", "<"], Self::parse_add_sub)
    }

    fn parse_add_sub(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["+", "-"], Self::parse_mul_div)
    }

    fn parse_mul_div(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["*", "/"], Self::parse_pow)
    }

    fn parse_pow(&self, expr: &str) -> EvalResult {
        self.parse_binary(expr, &["^"], Self::parse_atom)
    }

    fn parse_binary(
        &self,
        expr: &str,
        ops: &[&str],
        next: fn(&Self, &str) -> EvalResult,
    ) -> EvalResult {
        let mut depth = 0;
        let mut i = 0;
        while i < expr.len() {
            if &expr[i..=i] == "(" {
                depth += 1;
            } else if &expr[i..=i] == ")" {
                depth -= 1;
            }

            for &op in ops {
                if depth == 0 && expr[i..].starts_with(op) {
                    let (left, right) = expr.split_at(i);
                    let right = &right[op.len()..];
                    let l = next(self, left.trim())?;
                    let r = next(self, right.trim())?;
                    return match op {
                        "+" => Ok(l + r),
                        "-" => Ok(l - r),
                        "*" => Ok(l * r),
                        "/" => if r == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        },
                        "^" => Ok(l.powf(r)),
                        ">" => Ok((l > r) as i32 as f64),
                        "<" => Ok((l < r) as i32 as f64),
                        ">=" => Ok((l >= r) as i32 as f64),
                        "<=" => Ok((l <= r) as i32 as f64),
                        "==" => Ok(((l - r).abs() < 1e-6) as i32 as f64),
                        "!=" => Ok(((l - r).abs() >= 1e-6) as i32 as f64),
                        "&&" => Ok(((l.abs() > 0.5) && (r.abs() > 0.5)) as i32 as f64),
                        "||" => Ok(((l.abs() > 0.5) || (r.abs() > 0.5)) as i32 as f64),
                        _ => Err(EvalError::InvalidSyntax(format!("Unknown operator: {op}")))
                    };
                }
            }
            i += 1;
        }

        next(self, expr)
    }

    fn parse_atom(&self, expr: &str) -> EvalResult {
        let expr = expr.trim();
    
        if expr.starts_with('(') && expr.ends_with(')') {
            return self.parse_expression(&expr[1..expr.len() - 1]);
        }
    
        if let Ok(v) = f64::from_str(expr) {
            return Ok(v);
        }
    
        if let Some(open_paren) = expr.find('(') {
            let func_name = &expr[..open_paren].trim().to_uppercase();
            let arg_str = &expr[open_paren + 1..expr.len() - 1]; // remove closing ')'
    
            let args = Self::split_args(arg_str)?;
            let parsed_args: Result<Vec<f64>, EvalError> = args.iter().map(|arg| self.parse_expression(arg)).collect();
            let args = parsed_args?;
    
            return match func_name.as_str() {
                "MIN" => args.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).ok_or(EvalError::MissingArguments),
                "MAX" => args.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).ok_or(EvalError::MissingArguments),
                "AVERAGE" => {
                    if args.is_empty() {
                        Err(EvalError::MissingArguments)
                    } else {
                        Ok(args.iter().sum::<f64>() / args.len() as f64)
                    }
                }
                "IF" => {
                    if args.len() != 3 {
                        Err(EvalError::FunctionError("IF requires 3 arguments".into()))
                    } else {
                        Ok(if args[0].abs() > 0.5 { args[1] } else { args[2] })
                    }
                }
                "POW" => {
                    if args.len() != 2 {
                        Err(EvalError::FunctionError("POW requires 2 arguments".into()))
                    } else {
                        Ok(args[0].powf(args[1]))
                    }
                }
                _ => Err(EvalError::FunctionError(format!("Unknown function '{func_name}'"))),
            };
        }
    
        for (key, val) in &self.player.stats {
            if key.eq_ignore_ascii_case(expr) {
                return Ok(*val as f64);
            }
        }
    
        Err(EvalError::UnknownStat(expr.to_string()))
    }

    fn split_args(input: &str) -> Result<Vec<&str>, EvalError> {
        let mut args = Vec::new();
        let mut depth = 0;
        let mut last = 0;

        for (i, c) in input.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    if depth == 0 {
                        return Err(EvalError::InvalidSyntax("Mismatched parentheses".into()));
                    }
                    depth -= 1;
                }
                ',' if depth == 0 => {
                    args.push(input[last..i].trim());
                    last = i + 1;
                }
                _ => {}
            }
        }

        if last < input.len() {
            args.push(input[last..].trim());
        }

        Ok(args)
    }
}

use crate::composition::PositionRequirements;

pub fn evaluate_position(player: &Player, position: &str, reqs: &PositionRequirements) -> EvalResult {
    let expr = reqs
        .position_to_calculation
        .get(position)
        .map(|s| s.as_str())
        .unwrap_or(position); // default to position name if no formula

    evaluate(player, expr)
}


