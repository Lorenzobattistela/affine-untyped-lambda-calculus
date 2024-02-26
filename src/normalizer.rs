use crate::parser::Term;

// normalizer should implement beta reduction, where a lambda application is reduced:
// Lambda application:
//  ((λx. body) arg) ~> body[x <~ arg]
//a ~> b    means a reduces to b
// a[b <~ c] means all occurrences of b in a are replaced by c

pub fn substitute(term: Term, var: char, replacement: Term) -> Term {
    match term {
        Term::Variable(v) if v == var => replacement,
        Term::Variable(_) => term,
        Term::Function(param, body) => {
            if let Term::Variable(p) = *param {
                if p != var {
                    Term::Function(param, Box::new(substitute(*body, var, replacement)))
                } else {
                    Term::Function(param, body)
                }
            } else {
                unreachable!() // Assuming all params are Variables
            }
        }
        Term::Application(left, right) => Term::Application(
            Box::new(substitute(*left, var, replacement.clone())),
            Box::new(substitute(*right, var, replacement)),
        ),
    }
}

pub fn normalize(term: Term) -> Term {
    match term {
        Term::Variable(_) => term,
        Term::Function(param, body) => Term::Function(param, Box::new(normalize(*body))),
        Term::Application(left, right) => {
            match *left {
                Term::Function(param, body) => {
                    // This is a redex, perform substitution and continue normalizing
                    normalize(substitute(
                        *body,
                        if let Term::Variable(v) = *param {
                            v
                        } else {
                            unreachable!()
                        },
                        *right,
                    ))
                }
                _ => {
                    // If the left part is not a function, just normalize both sides
                    let left_norm = normalize(*left);
                    let right_norm = normalize(*right);
                    // After normalizing both sides, we need to check if normalization has
                    // turned the left part into a function, thus creating a redex.
                    if let Term::Function(_, _) = left_norm {
                        normalize(Term::Application(Box::new(left_norm), Box::new(right_norm)))
                    } else {
                        Term::Application(Box::new(left_norm), Box::new(right_norm))
                    }
                }
            }
        }
    }
}
