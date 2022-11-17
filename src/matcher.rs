use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};
use regex::{Regex, Error};

pub struct Matcher {
    near_block_regex: Option<Regex>,

    aurora_block_regex: Option<Regex>,
}

impl Matcher {

    pub fn new (near_block_expression: Option<String>, aurora_block_expression: Option<String>) -> Result<Self, Error> {

        let near_block_regex = Matcher::try_build_regex(near_block_expression)?;

        let aurora_block_regex = Matcher::try_build_regex(aurora_block_expression)?;

        Ok(Self { near_block_regex, aurora_block_regex })
    }

    pub fn matches(&self, near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>) -> bool {
        
        // near block condition
        let near_block_condition = match &self.near_block_regex {
            None => true,
            Some(regex) =>
            {
                let near_block_data = serde_json::to_string(&near_block).unwrap();
                regex.is_match(&near_block_data)
            }
        };

        // aurora blocks condition (any of them)
        let aurora_blocks_condition = match &self.aurora_block_regex {
            None => true,
            Some(regex) =>
            {
                let mut match_found = false;

                for aurora_block in aurora_blocks {

                    let aurora_block_data = serde_json::to_string(&aurora_block).unwrap();
                    if regex.is_match(&aurora_block_data)
                    {
                        match_found = true;
                        break;
                    }
                }

                match_found
            }
        };

        near_block_condition && aurora_blocks_condition
    }

    fn try_build_regex(expression: Option<String>) -> Result<Option<Regex>, Error> {

        // Scenarios:
        // expression is none => ok none
        // expression is some string but not a regex => error
        // expression is some string and a regex => ok regex

        match expression {
            None => Ok(None),
            Some(s) => {
                let regex = Regex::new(&s);
                match regex {
                    Err(e) => Err(e),
                    Ok(regex) => Ok(Some(regex))
                }
            }
        }
    }
}