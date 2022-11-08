mod cli;
mod config;
mod conversion;
mod input;
mod matcher;

use clap::Parser;
use cli::Cli;
use config::{InputMode, OutputStoreConfig};

use aurora_standalone_engine::EngineContext;
use aurora_refiner_lib::{self, near_stream::NearStream};
use aurora_refiner_types::{near_block::NEARBlock, aurora_block::AuroraBlock};

use std::path::{PathBuf};
use tokio::io::AsyncWriteExt;

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

    // Get start and end block heights
    let height_start = args.near_block_height_start;
    let (height_last, height_next) = if let Some(height_start) = height_start {
        (height_start.checked_sub(1), height_start)
    } else {
        (None, config.height_start_default)
    };

    let height_end = args.near_block_height_end.unwrap_or(u64::MAX);

    // Build input stream
    let mut input_stream = match config.input_mode {
        InputMode::DataLake(config) => {
            input::data_lake::get_near_data_lake_stream(height_next, &config)
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
        height_last,
        ctx);

    // Build matcher
    let matcher = matcher::Matcher::new(args.near_block_expression, args.aurora_block_expression);

    // Process
    while let Some(message) = input_stream.recv().await {

        let near_block = message.block;

        if near_block.block.header.height > height_end {
            break;
        }

        let aurora_blocks = near_to_aurora_stream.next_block(&near_block);

        if matcher.matches(&near_block, &aurora_blocks)
        {
            let path_buf = save_output(&near_block, &aurora_blocks, &config.output_storage).await;

            println!("Finder found match at near block height: {}", near_block.block.header.height);
            println!("Output is on file: {}", path_buf.into_os_string().into_string().unwrap());
            return;
        }
    }

    println!("No match was found for your search.");
}

async fn save_output(near_block: &NEARBlock, aurora_blocks: &Vec<AuroraBlock>, output_store_config: &OutputStoreConfig) -> PathBuf {

    let folder_path = std::path::PathBuf::from(&output_store_config.path);

    if !folder_path.exists() {
        std::fs::create_dir_all(&folder_path).unwrap();
    }

    let mut tmp_path = folder_path.clone();
    tmp_path.push(".PARTIAL");

    let file = tokio::fs::File::create(&tmp_path).await.unwrap();

    {
        let mut writer = tokio::io::BufWriter::new(file);
        let data = serde_json::to_string(&(near_block, aurora_blocks)).unwrap();
        writer.write_all(data.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }

    let mut target_path = folder_path;
    target_path.push(format!("{}.json", near_block.block.header.height));

    tokio::fs::rename(tmp_path, &target_path).await.unwrap();

    target_path
}