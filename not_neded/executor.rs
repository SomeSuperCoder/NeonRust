// use crate::{instruction::Instruction, program::ProgramTrait, program_runtime::ProgramRuntime};
// use std::rc::Rc;

// pub struct Executor {
//     instruction: Instruction,
//     underlying_process: std::thread::JoinHandle<()>,
//     parent: Rc<ProgramRuntime>
// }

// impl Executor {
//     pub fn new<T: ProgramTrait>(program: T, instruction: Instruction, parent: Rc<ProgramRuntime>) -> Self {
//         Self {
//             instruction,
//             parent,
//             underlying_process: std::thread::spawn(|| {})
//         }
//     }

//     pub fn join(self) -> Result<(), Box<(dyn std::any::Any + Send + 'static)>> {
//         self.underlying_process.join()
//     }
// }
