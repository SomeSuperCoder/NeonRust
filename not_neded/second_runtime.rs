use core::panic;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::instruction::Instruction;
use crate::program::{process_builtin_address, ProgramTrait};
use crate::wrapper::Wrapper;

pub struct Runtime {
    executor_list: Vec<Executor>,
    todo: Mutex<Vec<Instruction>>,
    will_read: WillRead,
    will_write: WillWrite,
    invoke_handler: Option<Arc<Mutex<InvokeHandler>>>,
    parent: Arc<Wrapper>
}

impl Runtime {
    pub fn feed(&mut self, instrcution_list: Vec<Instruction>) {
        for instruction in instrcution_list {
            match &self.invoke_handler {
                Some(obj) => {
                    let mut access = obj.lock().unwrap();
                    access.invoke(instruction);
                    drop(access)
                }
                None => panic!("!!! runtime.invoke_handler can't be None !!!")
            }
        }        
    }

    pub fn new(parent: Arc<Wrapper>) -> Arc<Self> {
        let mut result = Arc::new(
            Self {
                executor_list: Vec::new(),
                parent: parent,
                todo: Mutex::from(Vec::new()),
                will_read: WillRead::default(),
                will_write: WillWrite::default(),
                invoke_handler: None
            }
        );

        result.invoke_handler = Some(
            Arc::new(Mutex::from(InvokeHandler {
                parent: Arc::clone(&result)
            }))
        );

        result
    }
}

#[derive(Default)]
pub struct WillWrite {
    data: HashMap<usize, Vec<String>>
}

#[derive(Default)]
pub struct WillRead {
    data: HashMap<usize, Vec<String>>
}

impl WillWrite {
    fn conflicts(&self, other: Vec<String>) -> bool {
        for (key, value )in self.data.clone().into_iter() {
            for other_write in other.clone().into_iter(){
                if value.contains(&other_write) {
                    return true;
                }
            }
        }

        false
    }
}

impl WillRead {
    fn conflicts(&self, other: Vec<String>) -> bool {
        for (key, value )in self.data.clone().into_iter() {
            for other_write in other.clone().into_iter(){
                if value.contains(&other_write) {
                    return true;
                }
            }
        }

        false
    }
}

pub struct InvokeHandler {
    parent: Arc<Runtime>,
}

impl InvokeHandler {
    pub fn invoke(&mut self, instrcution: Instruction) {
        let program = crate::program::process_builtin_address(instrcution.program_id.as_str());
        match program {
            Some(program) => {
                match program.will_write(&instrcution, &self.parent.parent.cache) {
                    Ok(will_write) => {
                        if self.parent.will_write.conflicts(will_write.clone()) ||
                        self.parent.will_read.conflicts(will_write.clone()) {
                            let mut access = self.parent.todo.lock().unwrap();
                            access.push(instrcution);
                            drop(access)
                        } else {
                            match program.will_read(&instrcution, &self.parent.parent.cache) {
                                Ok(will_read) => {
                                    let will_read = will_read.to_will_read();
                                    if self.parent.will_write.conflicts(will_read){
                                        let mut access = self.parent.todo.lock().unwrap();
                                        access.push(instrcution);
                                        drop(access)
                                    } else {
                                        todo!("Add executor in InvokeHandler");
                                    }
                                },
                                Err(will_read) => {}
                            }
                        }
                    },
                    Err(will_write) => {}
                }
            },
            None => {}
        }
    }
}

struct Executor {
    process: JoinHandle<()>,
    parent: Arc<Runtime>
}

impl Executor {
    pub fn from(instruction: Instruction, parent: Arc<Runtime>) -> Self {
        match process_builtin_address(&instruction.program_id) {
            Some(program) => {
                Self {
                    parent: Arc::clone(&parent),
                    process: thread::spawn(
                        move || {
                            program.execute(
                                instruction,
                                Arc::clone(&parent.parent.cache),
                                Arc::clone(&parent.invoke_handler)
                            ).expect("The system program crashed and you did not implement handling this yet!");
                        }
                    )
                }
            },
            None => {
                panic!("Executor cannot process this. This shold have been fltered in the InvokeHandler::invoke function by got later passed in to the Executor::from function")
            }
        }
    }
}

pub struct ProgramRequire {
    data: HashMap<String, String>
}

impl ProgramRequire {
    pub fn new(from: Vec<(String, String)>) -> Self {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        for (key, value) in from {
            hash_map.insert(key, value);
        }
        Self {
            data: hash_map
        }
    }

    pub fn to_will_read(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (key, value) in &self.data {
            result.push(key.clone())
        }

        result
    }
}
