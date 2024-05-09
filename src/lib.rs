//! 应用接口

// todo 删除这行
#![allow(unused)]

mod cli;
mod model;
mod service;
mod storage;
mod util;

use clap::Parser;
use cli::Cli;
use service::{TodoService, TodoStatus};

pub fn run() {
    let cli = Cli::parse();
    let mut service = TodoService::new();
    service.init();

    if let Some(name) = cli.add {
        service.add_todo(name.as_str());
    }

    if let Some(olist) = cli.list {
        match olist {
            Some(status) => {
                match status.as_str() {
                    "all" | "a" => {
                        service.show_todo_list(TodoStatus::All);
                    }
                    "completed" | "c" => {
                        service.show_todo_list(TodoStatus::Completed);
                    }
                    "uncompleted" | "u" => {
                        service.show_todo_list(TodoStatus::Uncompleted);
                    }
                    "deleted" | "d" => {
                        service.show_todo_list(TodoStatus::Deleted);
                    }
                    _ => {
                        eprintln!("参数有误，请重新输入：[a all | c completed | u uncompleted | d deleted]");
                    }
                }
            }
            None => {
                service.show_todo_list(TodoStatus::All);
            }
        }
    }

    if let Some(id) = cli.uncomplete {
        service.uncomplete_todo(id.parse().expect("id 必须是数字"));
    }

    if let Some(id) = cli.delete {
        service.delete_todo(id.parse().expect("id 必须是数字"));
    }

    if let Some(id) = cli.restore {
        service.restore_todo(id.parse().expect("id 必须是数字"));
    }

    if let Some(id) = cli.destroy {
        service.destroy_todo(id.parse().expect("id 必须是数字"));
    }

    if let Some(b) = cli.destroy_deleted {
        if b {
            service.destroy_deleted_todo();
        }
    }

    if let Some(b) = cli.clear {
        if b {
            service.clear_todo();
        }
    }
}
