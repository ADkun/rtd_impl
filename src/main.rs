//! 命令行处理
//!
//! - 添加一个todo
//! rtd -a <item-name>
//! rtd --add <item-name>
//!
//! - 列出所有未完成的todo
//! rtd -l
//! rtd --list
//! rtd -l uncompleted
//! rtd --list uncompleted

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version = "1.0", about = "adkun", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "ITEM-NAME", help = "Add a new todo")]
    add: Option<String>,

    #[arg(
        short,
        long,
        value_name = "STATUS",
        help = "Get todos by status [all, completed, uncompleted]"
    )]
    list: Option<Option<String>>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(_) = cli.list {
        println!("has list");
    }
}
