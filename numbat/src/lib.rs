mod arithmetic;
mod ast;
mod bytecode_interpreter;
mod decorator;
mod dimension;
mod ffi;
mod interpreter;
mod name_resolution;
mod number;
mod parser;
mod prefix;
mod prefix_parser;
mod prefix_transformer;
pub mod pretty_print;
mod product;
mod quantity;
mod registry;
mod span;
mod tokenizer;
mod typechecker;
mod typed_ast;
mod unit;
mod unit_registry;
mod vm;

use bytecode_interpreter::BytecodeInterpreter;
use interpreter::{Interpreter, RuntimeError};
use name_resolution::NameResolutionError;
use parser::parse;
use prefix_transformer::Transformer;
use thiserror::Error;
use typechecker::{TypeCheckError, TypeChecker};

use ast::Statement;
pub use interpreter::ExitStatus;
pub use interpreter::InterpreterResult;
pub use parser::ParseError;

#[derive(Debug, Error)]
pub enum NumbatError {
    #[error("{0}")]
    ParseError(ParseError),
    #[error("{0}")]
    NameResolutionError(NameResolutionError),
    #[error("{0}")]
    TypeCheckError(TypeCheckError),
    #[error("{0}")]
    RuntimeError(RuntimeError),
}

pub type Result<T> = std::result::Result<T, NumbatError>;

pub struct Numbat {
    prefix_transformer: Transformer,
    typechecker: TypeChecker,
    interpreter: BytecodeInterpreter,
}

impl Numbat {
    pub fn new_without_prelude(debug: bool) -> Self {
        Numbat {
            prefix_transformer: Transformer::new(),
            typechecker: TypeChecker::default(),
            interpreter: BytecodeInterpreter::new(debug),
        }
    }

    pub fn new(debug: bool) -> Self {
        let mut numbat = Self::new_without_prelude(debug);
        assert!(numbat
            .interpret(include_str!("../../prelude.nbt"))
            .expect("Error while running prelude")
            .1
            .is_success()); // TODO: read prelude dynamically, error handling
        numbat
    }

    pub fn interpret(&mut self, code: &str) -> Result<(Vec<Statement>, InterpreterResult)> {
        let statements = parse(code).map_err(NumbatError::ParseError)?;
        let transformed_statements = self
            .prefix_transformer
            .transform(statements)
            .map_err(NumbatError::NameResolutionError)?;
        let typed_statements = self
            .typechecker
            .check_statements(transformed_statements.clone()) // TODO: get rid of clone?
            .map_err(NumbatError::TypeCheckError)?;
        let result = self
            .interpreter
            .interpret_statements(&typed_statements)
            .map_err(NumbatError::RuntimeError)?;

        Ok((transformed_statements, result))
    }
}