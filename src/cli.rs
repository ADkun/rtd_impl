//! 命令行处理器

use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(version = "1.0", about = "adkun", long_about = None)]
#[command(group(ArgGroup::new("action")
    .args(&["add", "list", "uncomplete", "delete", "restore", "destroy", "destroy_deleted", "clear"])
    .required(false)
    .multiple(false)))]
pub struct Cli {
    /// Add a todo
    #[arg(short, long, value_name = "ITEM-NAME", help = "Add a new todo")]
    pub add: Option<String>,

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
    pub list: Option<Option<String>>,

    /// Mark a todo as uncompleted
    #[arg(short, long, value_name = "ID", help = "Mark a todo as uncompleted")]
    pub uncomplete: Option<String>,

    /// Delete a todo into trash bin
    #[arg(
        short,
        long = "delete",
        value_name = "ID",
        help = "Delete a todo into trash bin"
    )]
    pub delete: Option<String>,

    /// Restore a deleted todo
    #[arg(
        long,
        value_name = "ID",
        help = "Restore a deleted todo from trash bin"
    )]
    pub restore: Option<String>,

    /// Destroy a todo permanently
    #[arg(
        long = "destroy",
        value_name = "ID",
        help = "Destroy a todo permanently"
    )]
    pub destroy: Option<String>,

    /// Clean up the trash bin
    #[arg(long = "destroy-deleted", help = "Clean up the trash bin")]
    pub destroy_deleted: Option<bool>,

    #[arg(long, help = "Clear all todos")]
    pub clear: Option<bool>,
}
