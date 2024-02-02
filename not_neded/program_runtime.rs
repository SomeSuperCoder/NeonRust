// use crate::program::process_builtin_address;
// use crate::transaction::Transaction;
// use crate::{process_order::*, instruction};
// use crate::executor::Executor;
// use crate::instruction::Instruction;
// use std::rc::Rc;

// #[derive(Default)]
// pub struct ProgramRuntime {
//     process_order: ProcessOrder,
//     executors: Vec<Executor>
// }

// impl ProgramRuntime {
//     pub fn invoke_instrcution(&mut self, instruction: Instruction) {
//         let new_executor = Executor::new (
//             process_builtin_address(&instruction.program_id),
//             Rc::from(self)
//         );
//     }

//     pub fn feed_list(&mut self, tx_list: Vec<Transaction>) {
//         for tx in tx_list {
//             self.process_order.feed(tx);
//         }
//     }

//     pub fn execute(&mut self) {
//         for layer in self.process_order.layers {
//             for tx in layer.tx_list {
//                 let instruction = Instruction::from_tx(&tx);
//                 self.invoke_instrcution(instruction)
//             }
//         }
//     }
// }
