use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Args {
    #[arg(short, long)]
    pub bake: Option<PathBuf>,
    #[arg(short, long)]
    pub input: Option<PathBuf>,
}
