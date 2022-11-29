use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};
use jmespath::{Expression, JmespathError, Runtime};

pub struct Matcher<'a>{

    runtime: Runtime,

    near_block_expression: Option<Expression<'a>>,

    aurora_block_expression: Option<Expression<'a>>,
}

impl Matcher<'_> {

    pub fn new (near_block_expression: Option<String>, aurora_block_expression: Option<String>) -> Result<Self, JmespathError> {

        let runtime = jmespath::create_default_runtime();

        // near expression
        let near_block_expression = match near_block_expression {
            None => Ok(None),
            Some(s) => {
                let expression = runtime.compile(&s);
                match expression {
                    Err(e) => Err(e),
                    Ok(expression) => Ok(Some(expression))
                }
            }
        };

        let near_block_expression = near_block_expression?;

        // aurora expression
        let aurora_block_expression = match aurora_block_expression {
            None => Ok(None),
            Some(s) => {
                let expression = runtime.compile(&s);
                match expression {
                    Err(e) => Err(e),
                    Ok(expression) => Ok(Some(expression))
                }
            }
        };

        let aurora_block_expression = aurora_block_expression?;

        // matcher
        Ok(Self { runtime, near_block_expression, aurora_block_expression })
    }

    pub fn matches(&self, near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>) -> bool {
        
        // near block condition
        let near_block_condition = match &self.near_block_expression {
            None => true,
            Some(expression) =>
            {
                let json = serde_json::to_string(&near_block).unwrap();
                let variable = jmespath::Variable::from_json(&json).unwrap();
                let result = expression.search(variable).unwrap();
                result.is_truthy()
            }
        };

        // aurora blocks condition (any of them)
        let aurora_blocks_condition = match &self.aurora_block_expression {
            None => true,
            Some(expression) =>
            {
                let mut match_found = false;

                for aurora_block in aurora_blocks {

                    let json = serde_json::to_string(&aurora_block).unwrap();
                    let variable = jmespath::Variable::from_json(&json).unwrap();
                    let result = expression.search(variable).unwrap();

                    if result.is_truthy()
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
}