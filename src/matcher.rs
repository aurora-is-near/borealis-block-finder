use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};

pub struct Matcher {
    pub near_block_expression: Option<String>,

    pub aurora_block_expression: Option<String>,
}

impl Matcher {

    pub fn new (near_block_expression: Option<String>, aurora_block_expression: Option<String>) -> Self {
        Self { near_block_expression, aurora_block_expression }
    }

    pub fn matches(&self, near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>) -> bool {
        // ToDo
        true
    }
}