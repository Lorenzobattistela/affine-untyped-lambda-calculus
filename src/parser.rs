#[derive(Debug)]
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
}

// function: peek @, if it is @, consume @, then parse_variable, then consume . then parse term
// application: peek (, consume (, parse term, then consume an empty space, parse another term and
// consume )
// variable: is_variable, one letter vars.

impl Parser {
    pub fn new(source: Vec<char>) -> Self {
        Parser { source, current: 0 }
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
        println!("Parsing function!");
        self.consume('@');
        println!("Consumed @, parsing variable!");
        let variable: Term = self.parse_variable()?;
        variable.print();
        println!("Parsed variable!");
        self.consume('.');
        println!("Now parsing body.");
        let body: Term = self.parse_term()?;
        Some(Term::Function(Box::new(variable), Box::new(body)))
    }

    fn parse_variable(&mut self) -> Option<Term> {
        println!("Parsing variable.");
        let variable_name = self.peek();
        if !self.is_variable(variable_name) {
            panic!(
                "Variable: {}. Invalid variable name, pick a letter from a - z.",
                variable_name
            );
        }
        println!("Consuming variable name: {}.", variable_name);
        self.consume(variable_name);
        //println!("Current on parser is: {}", self.peek());
        Some(Term::Variable(variable_name))
    }

    fn parse_application(&mut self) -> Option<Term> {
        println!("Parsing application!");
        println!("Current is: {}", self.peek());
        self.consume('(');
        let first_term = self.parse_term()?;
        self.consume(' ');
        let second_term = self.parse_term()?;
        println!(
            "Parsed second term of application. Going to consume ')'. Position is: {}",
            self.current
        );
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
