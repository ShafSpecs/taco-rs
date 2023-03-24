use crate::{
    core::{expression::Expr, literal::Literal},
    environment::environment::Environment,
    error::{interpreter::RuntimeError, parser::ParserError},
    lang::taco::Taco,
    token::tokens::TokenType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Integer(i) => Value::Integer(i),
            Literal::Float(f) => Value::Float(f),
            Literal::String(s) => Value::String(s),
            Literal::Boolean(b) => Value::Boolean(b),
            Literal::Nil => Value::Nil,
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    had_error: bool,
    had_runtime_error: bool,
}

impl Interpreter {
    pub fn new(instance: &Taco) -> Interpreter {
        Interpreter {
            had_error: instance.has_error,
            had_runtime_error: instance.has_runtime_error,
        }
    }

    // pub fn run_file(&mut self, path: &str) {
    //     let source = std::fs::read_to_string(path).unwrap();
    //     self.run(source);
    //     if self.had_error {
    //         std::process::exit(65);
    //     }
    //     if self.had_runtime_error {
    //         std::process::exit(70);
    //     }
    // }

    // pub fn run_prompt(&mut self) {
    //     use std::io::{self, Write};
    //     loop {
    //         print!("> ");
    //         io::stdout().flush().unwrap();
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         self.run(input);
    //         self.had_error = false;
    //     }
    // }

    // fn run(&mut self, source: String) {
    //     let tokens = scanner::scan(source);
    //     let statements = parser::parse(tokens);
    //     if self.had_error {
    //         return;
    //     }
    //     self.interpret(statements);
    // }

    pub fn stringify(&self, value: Value) -> String {
        match value {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s,
            Value::Boolean(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
        }
    }

    pub fn interpret(&self, ast: Result<Expr, ParserError>) -> Result<(), RuntimeError> {
        if self.had_error {
            // todo!("Handle this better")
            return Ok(());
        }

        let statements = match ast {
            Ok(expr) => expr,
            Err(err) => {
                println!("{:?}", err);
                // todo!("Handle this better")
                return Ok(());
            }
        };

        let mut environment = Environment::new();
        self.execute(&mut environment, statements);
        Ok(())
    }

    fn execute(&self, environment: &mut Environment, statement: Expr) {
        let val = self.evaluate(environment, statement.clone());
        println!("{:?}", self.stringify(val));
        // match statement {
        // Expr::Expression(expr) => {
        //     let value = self.evaluate(environment, *expr);
        //     println!("{}", self.stringify(value));
        // }
        // Expr::VariableDeclaration(name, expr) => {
        //     let value = self.evaluate(environment, *expr);
        //     environment.define(name, value);
        // }
        // Expr::VariableAssignment(name, expr) => {
        //     let value = self.evaluate(environment, *expr);
        //     environment.assign(name, value);
        // }
        // Expr::Block(statements) => {
        //     let mut block_environment = Environment::new();
        //     block_environment.enclosing = Some(environment);
        //     for statement in statements {
        //         self.execute(&mut block_environment, statement);
        //     }
        // }
        // Expr::If(condition, then_branch, else_branch) => {
        //     let condition_value = self.evaluate(environment, *condition);
        //     if condition_value.is_truthy() {
        //         self.execute(environment, *then_branch);
        //     } else if let Some(else_branch) = else_branch {
        //         self.execute(environment, *else_branch);
        //     }
        // }
        // Expr::While(condition, body) => {
        //     while self.evaluate(environment, *condition).is_truthy() {
        //         self.execute(environment, *body);
        //     }
        // }
        // Expr::Function(_, _, _) => todo!(),
        // Expr::Call(_, _) => todo!(),
        // Expr::Return(_, _) => todo!(),
        // Expr::Class(_, _, _) => todo!(),
        // Expr::Get(_, _) => todo!(),
        // Expr::Set(_, _, _) => todo!(),
        // Expr::This(_) => todo!(),
        // Expr::Super(_, _) => todo!(),
        // _ => todo!(),
        // }
    }

    // fn execute(&self, environment: &mut environment::Environment, statement: parser::Statement) {
    //     match statement {
    //         parser::Statement::Expression(expr) => {
    //             let value = self.evaluate(environment, expr);
    //             println!("{}", value);
    //         }
    //         parser::Statement::VariableDeclaration(name, expr) => {
    //             let value = self.evaluate(environment, expr);
    //             environment.define(name, value);
    //         }
    //         parser::Statement::VariableAssignment(name, expr) => {
    //             let value = self.evaluate(environment, expr);
    //             environment.assign(name, value);
    //         }
    //         parser::Statement::Block(statements) => {
    //             let mut block_environment = environment::Environment::new();
    //             block_environment.enclosing = Some(environment);
    //             for statement in statements {
    //                 self.execute(&mut block_environment, statement);
    //             }
    //         }
    //         parser::Statement::If(condition, then_branch, else_branch) => {
    //             let condition_value = self.evaluate(environment, condition);
    //             if condition_value.is_truthy() {
    //                 self.execute(environment, *then_branch);
    //             } else if let Some(else_branch) = else_branch {
    //                 self.execute(environment, *else_branch);
    //             }
    //         }
    //         parser::Statement::While(condition, body) => {
    //             while self.evaluate(environment, condition).is_truthy() {
    //                 self.execute(environment, *body.clone());
    //             }
    //         }
    //         parser::Statement::FunctionDeclaration(name, parameters, body) => {
    //             let function = value::Value::Function(value::Function::new(
    //                 name,
    //                 parameters,
    //                 body,
    //                 environment.clone(),
    //             ));
    //             environment.define(name, function);
    //         }
    //         parser::Statement::Return(expr) => {
    //             let value = self.evaluate(environment, expr);
    //             panic!(value);
    //         }
    //     }
    // }

    fn evaluate(&self, environment: &mut Environment, expr: Expr) -> Value {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::String(string) => Value::String(string),
                Literal::Integer(integer) => Value::Integer(integer),
                Literal::Float(float) => Value::Float(float),
                Literal::Boolean(boolean) => Value::Boolean(boolean),
                Literal::Nil => Value::Nil,
            },
            Expr::GroupingExpr(expr) => self.evaluate(environment, expr.expr.into()),
            Expr::UnaryExpr(unary) => {
                let unary_expr = *unary;
                let operator = unary_expr.get_operator().clone();
                let expr = unary_expr.get_right().clone();

                let right = self.evaluate(environment, expr);
                match operator.token_type {
                    TokenType::Minus => match right {
                        Value::Integer(integer) => Value::Integer(-integer),
                        Value::Float(float) => Value::Float(-float),
                        _ => panic!("Invalid operand for unary minus"),
                    },
                    TokenType::Bang => Value::Boolean(!right.is_truthy()),
                    _ => panic!("Invalid unary operator"),
                }
            }
            Expr::BinaryExpr(binary) => {
                let left = binary.get_left();
                let operator = binary.get_operator().clone();
                let right = binary.get_right();

                let left = self.evaluate(environment, left);
                let right = self.evaluate(environment, right);
                match operator.token_type {
                    TokenType::Plus => match (left.clone(), right.clone()) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left + right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Float(left + right),
                        (Value::String(left), Value::String(right)) => Value::String(left + &right),
                        _ => panic!(
                            "Invalid operands for addition: {:?} - {:?}",
                            left.clone(),
                            right.clone()
                        ),
                    },
                    TokenType::Minus => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left - right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Float(left - right),
                        _ => panic!("Invalid operands for subtraction"),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Integer(left * right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Float(left * right),
                        _ => panic!("Invalid operands for multiplication"),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            if right.eq(&i64::from(0)) {
                                panic!("Division by zero");
                            }

                            Value::Integer(left / right)
                        }
                        (Value::Float(left), Value::Float(right)) => {
                            if right.eq(&f64::from(0)) {
                                panic!("Division by zero");
                            }
                            
                            Value::Float(left / right)
                        }
                        _ => panic!("Invalid operands for division"),
                    },
                    TokenType::Greater => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Boolean(left > right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left > right),
                        _ => panic!("Invalid operands for greater than"),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Boolean(left >= right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left >= right),
                        _ => panic!("Invalid operands for greater than or equal"),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Boolean(left < right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left < right),
                        _ => panic!("Invalid operands for less than"),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Value::Integer(left), Value::Integer(right)) => {
                            Value::Boolean(left <= right)
                        }
                        (Value::Float(left), Value::Float(right)) => Value::Boolean(left <= right),
                        _ => panic!("Invalid operands for less than or equal"),
                    },
                    TokenType::EqualEqual => match (left, right) {
                        (Value::Boolean(right), Value::Boolean(left)) => {
                            Value::Boolean(left == right)
                        }
                        (Value::Integer(right), Value::Integer(left)) => {
                            Value::Boolean(left.eq(&right))
                        }
                        (Value::Float(right), Value::Float(left)) => {
                            Value::Boolean(left.eq(&right))
                        }
                        (Value::String(right), Value::String(left)) => {
                            Value::Boolean(left.eq(&right))
                        }
                        _ => panic!("Invalid operands for equality"),
                    },
                    TokenType::BangEqual => match (left, right) {
                        (Value::Boolean(right), Value::Boolean(left)) => {
                            Value::Boolean(left != right)
                        }
                        (Value::Integer(right), Value::Integer(left)) => {
                            Value::Boolean(!left.eq(&right))
                        }
                        (Value::Float(right), Value::Float(left)) => {
                            Value::Boolean(!left.eq(&right))
                        }
                        (Value::String(right), Value::String(left)) => {
                            Value::Boolean(!left.eq(&right))
                        }
                        _ => panic!("Invalid operands for inequality"),
                    },
                    _ => panic!("Invalid binary operator"),
                }
            }
        }
    }
}
