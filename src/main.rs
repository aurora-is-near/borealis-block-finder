mod cli;
mod config;
mod conversion;
mod input;
mod matcher;

use std::path::PathBuf;

use clap::Parser;
use cli::Cli;

use aurora_standalone_engine::EngineContext;
use aurora_refiner_lib::{self, BlockWithMetadata, near_stream::NearStream};
use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};

#[tokio::main]
async fn main() {

    let args: Cli = Cli::parse();

    // Get config
    let config_path = args.config_path.as_deref().unwrap_or("default_config.json");
    let config: config::Config = {
        let file = std::fs::File::open(config_path).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    // Get start block
    let height = args.height;
    let (last_block, next_block) = if let Some(height) = height {
        (height.checked_sub(1), height)
    } else {
        (None, config.initial_height)
    };

    // Build input stream
    let mut input_stream = match config.input_mode {
        config::InputMode::DataLake(config) => {
            input::data_lake::get_near_data_lake_stream(next_block, &config)
        }
        _ => panic!("For the moment there is only support for DataLake InputMode.")
        };
        
    // Init storage
    aurora_refiner_lib::storage::init_storage(
        PathBuf::from(&config.refiner.engine_path),
        config.refiner.engine_account_id.parse().unwrap(),
        config.refiner.chain_id,
    );

    // Build near stream
    let ctx = EngineContext::new(
        config.refiner.engine_path,
        config.refiner.engine_account_id.parse().unwrap(),
        config.refiner.chain_id).unwrap();
    let mut near_to_aurora_stream = NearStream::new(
        config.refiner.chain_id,
        last_block,
        ctx);

    // Build matcher
    let matcher = matcher::Matcher::new(args.near_block_expression, args.aurora_block_expression);

    // Process
    // while let Some(message) = input_stream.recv().await {

    //     let near_block = message.block;
    //     let aurora_blocks = near_to_aurora_stream.next_block(near_block);

    //     if(matcher.matches(&near_block, &aurora_blocks))
    //     {
    //         save_output(&near_block, &aurora_blocks);
    //         println!("Finder found match at near block height: {}", near_block.block.header.height);
    //         println!("Output is on {}", config.output_storage.path);
    //         return;
    //     }
    // }

    println!("No match was found for your search.");
}

fn save_output(near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>) {
    // ToDo
}