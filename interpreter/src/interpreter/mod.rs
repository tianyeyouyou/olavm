mod executor;

use crate::interpreter::executor::Executor;
use crate::parser::node::Node;
use crate::parser::Parser;
use crate::sema::SymTableGen;
use crate::utils::number::NumberResult;
use core::{program::binary_program::OlaProphet, vm::hardware::OlaMemory};
use log::debug;
use std::sync::{Arc, RwLock};

pub struct Interpreter {
    pub root_node: Arc<RwLock<dyn Node>>,
}

impl Interpreter {
    pub fn new(text: &str) -> Self {
        let mut parser = Parser::new(&text);
        let root_node = parser.parse();
        Interpreter { root_node }
    }

    pub fn run(&mut self, prophet: &OlaProphet, values: Vec<u64>, mem: &OlaMemory) -> NumberResult {
        debug!("sema");
        self.root_node
            .write()
            .map_err(|err| format!("failed to lock write lock {}", err))?
            .traverse(&mut SymTableGen::new(&prophet))?;
        debug!("executor");
        let mut exe = Executor::new(&prophet, values, mem);
        self.root_node
            .write()
            .map_err(|err| format!("failed to lock write lock {}", err))?
            .traverse(&mut exe)
    }
}
