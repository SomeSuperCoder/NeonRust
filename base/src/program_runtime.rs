use crate::transaction::Transaction;
use crate::program::{process_builtin_address, ProgramTrait};

#[derive(Default)]
struct ProcessOrderLayer {
    will_touch: Vec<String>,
    tx_list: Vec<Transaction>
}

impl ProcessOrderLayer {
    pub fn add_will_touch(&mut self, will_touch: Vec<String>) {
        for i in will_touch {
            self.will_touch.push(i);
        }
    }
}

#[derive(Default)]
struct ProcessOrder {
    layers: Vec<ProcessOrderLayer>
}

impl ProcessOrder {
    pub fn feed(&mut self, tx: Transaction) {
        let target_program = process_builtin_address(tx.sender_part.program_id.as_str());
        match target_program {
            Some(target_program) => {
                let will_touch = target_program.will_touch(&tx);
                match will_touch {
                    Ok(will_touch) => {
                        self.add(tx, will_touch);
                    },
                    Err(_) => {}
                }
            },
            None => {}
        }
    }

    fn process_layering(&mut self, will_touch: &Vec<String>) -> usize {
        for (index, current_layer) in &mut self.layers.iter().enumerate() {
            for will_touch_element in will_touch {
                if current_layer.will_touch.contains(will_touch_element) {
                    continue;
                } else {
                    // Add to this layer if its ok!
                    return index
                }
            }
        }
        // In case if all layers are busy, just create a new one! (put this login in to an extra function)
        self.create_new_layer();
        self.layers.len() - 1
    }

    fn add(&mut self, tx: Transaction, will_touch: Vec<String>) {
        let target_index = self.process_layering(&will_touch);
        let target_layer = self.layers.get_mut(target_index).expect("Invalid layer index");
        target_layer.add_will_touch(will_touch);
        target_layer.tx_list.push(tx);
    }


    fn create_new_layer(&mut self) {
        self.layers.push(ProcessOrderLayer::default());
    }
}

#[derive(Default)]
pub struct ProgramRuntime {
    process_order: ProcessOrder
}

impl ProgramRuntime {
    pub fn feed_list(&mut self, tx_list: Vec<Transaction>) {
        for tx in tx_list {
            self.process_order.feed(tx);
        }
    }
}
