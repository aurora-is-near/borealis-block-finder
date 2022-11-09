use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};
use regex::{Regex, Error};

pub struct Matcher {
    near_block_regex: Regex,

    aurora_block_regex: Regex,
}

impl Matcher {

    pub fn new (near_block_expression: Option<String>, aurora_block_expression: Option<String>) -> Result<Self, Error> {

        let near_block_regex = near_block_expression.map_or(Regex::new(""), |s| Regex::new(&s));

        let near_block_regex = match near_block_regex {
            Ok(regex) => regex,
            Err(e) => return Err(e)
        };

        let aurora_block_regex = aurora_block_expression.map_or(Regex::new(""), |s| Regex::new(&s));

        let aurora_block_regex = match aurora_block_regex {
            Ok(regex) => regex,
            Err(e) => return Err(e)
        };

        Ok(Self { near_block_regex, aurora_block_regex })
    }

    pub fn matches(&self, near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>) -> bool {
        
        // test near block
        let near_block_data = serde_json::to_string(&near_block).unwrap();

        if self.near_block_regex.is_match(&near_block_data) {
            return true;
        }

        // test aurora blocks
        for aurora_block in aurora_blocks {

            let aurora_block_data = serde_json::to_string(&aurora_block).unwrap();

            if self.aurora_block_regex.is_match(&aurora_block_data) {
                return true;
            }
        }

        // defautl
        false
    }
}