use clap::{Parser};

//ToDo
// Know how to deal with parameter with the '_'
// Think in better names and/or shorts.
// Parameter height forced!

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

    /// [Optional] Path to config file.
    /// If this value is not passed the default config will be used.
    #[clap(short = 'c', long)]
    pub config_path: Option<String>,

    /// [Optional] Starts refiner at the specified height.
    /// If this value is not passed, first height will be used.
    #[clap(short = 's', long)]
    pub near_block_height_start: u64,

    /// [Optional] Ends refiner at the specified height.
    /// If this value is not passed, last height will be used.
    #[clap(short = 'e', long)]
    pub near_block_height_end: Option<u64>,

    /// [Optional] Near block expression to match.
    /// If this value is not passed, it will not be used to find matches.
    #[clap(short = 'n', long)]
    pub near_block_expression: Option<String>,

    /// [Optional] Aurora block expression to match.
    /// If this value is not passed, it will not be used to find matches.
    #[clap(short = 'a', long)]
    pub aurora_block_expression: Option<String>,
}
