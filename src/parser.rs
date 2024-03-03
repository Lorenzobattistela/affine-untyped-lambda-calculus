#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Term {
    // function term first should be a variable
    Function(Box<Term>, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Variable(char),
}

impl Term {
    pub fn print(&self) {
        match self {
            Term::Function(param, body) => {
                print!(
                    "(λ{}. ",
                    match **param {
                        Term::Variable(c) => c.to_string(),
                        _ => "".to_string(),
                    }
                );
                body.print();
                print!(")");
            }
            Term::Application(fun, arg) => {
                print!("(");
                fun.print();
                print!(" ");
                arg.print();
                print!(")");
            }
            Term::Variable(c) => {
                print!("{}", c);
            }
        }
    }
}

pub struct Parser {
    current: usize,
    source: Vec<char>,
    used_variables: HashMap<char, usize>,
}

// function: peek @, if it is @, consume @, then parse_variable, then consume . then parse term
// application: peek (, consume (, parse term, then consume an empty space, parse another term and
// consume )
// variable: is_variable, one letter vars.
// to make it affine, we need to make sure that variables are used at most once (but can -not be
// used)
// a possible approach is to mark a variable with a bool used and panic if variable is used more
// than once

impl Parser {
    pub fn new(source: Vec<char>) -> Self {
        Parser {
            source,
            current: 0,
            used_variables: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Option<Term> {
        let res = self.parse_term()?;
        Some(res)
    }

    fn parse_term(&mut self) -> Option<Term> {
        println!("Parsing term!");
        let next = self.peek();
        println!("Peeking next: {}", next);

        if next == '@' {
            return Some(self.parse_function()?);
        } else if next == '(' {
            let term = self.parse_application()?;
            return Some(term);
        } else if self.is_variable(next) {
            return Some(self.parse_variable()?);
        } else {
            panic!("Unexpected character: {}", next);
        }
    }

    fn parse_function(&mut self) -> Option<Term> {
        self.consume('@');
        let variable: Term = self.parse_variable()?;
        variable.print();
        self.consume('.');
        let body: Term = self.parse_term()?;
        Some(Term::Function(Box::new(variable), Box::new(body)))
    }

    fn parse_variable(&mut self) -> Option<Term> {
        let variable_name = self.peek();
        if !self.is_variable(variable_name) {
            panic!(
                "Variable: {}. Invalid variable name, pick a letter from a - z.",
                variable_name
            );
        }

        if self.used_variables.contains_key(&variable_name) {
            if let Some(x) = self.used_variables.get(&variable_name) {
                if *x != 1 {
                    panic!(
                        "Affine lambda calculus does not allow to use a variable more than once."
                    );
                }
                self.used_variables.insert(variable_name, *x + 1);
            }
        } else {
            self.used_variables.insert(variable_name, 1);
        }

        self.consume(variable_name);
        Some(Term::Variable(variable_name))
    }

    fn parse_application(&mut self) -> Option<Term> {
        self.consume('(');
        let first_term = self.parse_term()?;
        self.consume(' ');
        let second_term = self.parse_term()?;
        self.consume(')');
        Some(Term::Application(
            Box::new(first_term),
            Box::new(second_term),
        ))
    }

    fn consume(&mut self, expected: char) -> Option<char> {
        if self.peek() == expected {
            self.advance();
            Some(expected)
        } else {
            panic!(
                "Syntax error: expected {}, received {}",
                self.peek(),
                expected
            );
        }
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&self) -> char {
        return self.source[self.current - 1];
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            panic!("Input finished and tried to peek.");
        }
        return self.source[self.current];
    }

    fn peek_twice(&self) -> char {
        if self.is_at_end() {
            panic!("Input finished on peek twice.");
        }
        return self.source[self.current + 1];
    }

    fn is_at_end(&self) -> bool {
        return (self.current) == self.source.len();
    }

    fn is_variable(&self, c: char) -> bool {
        c >= 'a' && c <= 'z'
    }
}
