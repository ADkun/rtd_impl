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
    #[arg(short, long, value_name = "ITEM-NAME", help = "添加一个待办事项")]
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
        help = "获取待办事项，若为空则展示所有未删除待办事项，支持指定状态值：[all, completed, uncompleted, deleted]"
    )]
    pub list: Option<Option<String>>,

    /// Mark a todo as uncompleted
    #[arg(short, long, value_name = "ID", help = "标记一个待办事项为未完成")]
    pub uncomplete: Option<String>,

    /// Delete a todo into trash bin
    #[arg(
        short,
        long = "delete",
        value_name = "ID",
        help = "将一个待办事项放入回收站"
    )]
    pub delete: Option<String>,

    /// Restore a deleted todo
    #[arg(long, value_name = "ID", help = "将一个待办事项从回收站恢复")]
    pub restore: Option<String>,

    /// Destroy a todo permanently
    #[arg(long = "destroy", value_name = "ID", help = "彻底删除一个待办事项")]
    pub destroy: Option<String>,

    /// Clean up the trash bin
    #[arg(long = "destroy-deleted", help = "清空回收站")]
    pub destroy_deleted: Option<bool>,

    #[arg(long, help = "清空所有待办事项")]
    pub clear: Option<bool>,
}
