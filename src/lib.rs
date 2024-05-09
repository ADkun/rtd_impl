//! 应用接口

// todo 删除这行
#![allow(unused)]

mod model;
mod service;
mod storage;
mod util;

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(version = "1.0", about = "adkun", long_about = None)]
#[command(group(ArgGroup::new("action")
    .args(&["add", "list", "uncomplete", "delete", "restore", "destroy", "destroy_deleted", "clear"])
    .required(false)
    .multiple(false)))]
struct Cli {
    /// Add a todo
    #[arg(short, long, value_name = "ITEM-NAME", help = "Add a new todo")]
    add: Option<String>,

    /// Show all todos by status
    /// Supported status values:
    /// - empty（all）
    /// - completed
    /// - uncompleted
    /// - deleted
    #[arg(
        short,
        long = "list",
        value_name = "STATUS",
        help = "Get todos by status [empty(all), completed, uncompleted, deleted]"
    )]
    list: Option<Option<String>>,

    /// Mark a todo as uncompleted
    #[arg(
        short,
        long,
        value_name = "ITEM-NAME",
        help = "Mark a todo as uncompleted"
    )]
    uncomplete: Option<String>,

    /// Delete a todo into trash bin
    #[arg(
        short,
        long = "delete",
        value_name = "ITEM-NAME",
        help = "Delete a todo into trash bin"
    )]
    delete: Option<String>,

    /// Restore a deleted todo
    #[arg(
        long,
        value_name = "ITEM-NAME",
        help = "Restore a deleted todo from trash bin"
    )]
    restore: Option<String>,

    /// Destroy a todo permanently
    #[arg(
        long = "destroy",
        value_name = "ITEM-NAME",
        help = "Destroy a todo permanently"
    )]
    destroy: Option<String>,

    /// Clean up the trash bin
    #[arg(long = "destroy-deleted", help = "Clean up the trash bin")]
    destroy_deleted: Option<bool>,

    #[arg(long, help = "Clear all todos")]
    clear: Option<bool>,
}

pub fn run() {
    let cli = Cli::parse();

    if let Some(_) = cli.list {
        println!("has list");
    }
}
