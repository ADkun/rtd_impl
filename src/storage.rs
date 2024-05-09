//! 模型映射
//! - ORM
//! - 文件读写

use crate::model::{Todo, TodoList};
use std::error::Error;

/// 默认的CSV文件相对路径
pub const DEFAULT_CSV_REL_FILE_PATH: &str = "todo.csv";

/// 待办事项存储类型
pub enum TodoStorageType {
    Csv,
}

/// 待办事项存储工具类
/// 用于存取待办事项列表
pub struct TodoStorage {
    storage_type: TodoStorageType,
    file_path: String,
}

impl TodoStorage {
    pub fn new(storage_type: TodoStorageType, file_path: &str) -> Result<Self, Box<dyn Error>> {
        if file_path.is_empty() {
            return Err("file_path should not be empty".into());
        }

        Ok(Self {
            storage_type,
            file_path: file_path.to_string(),
        })
    }

    pub fn save(&self, todo_list: &TodoList) -> Result<(), Box<dyn Error>> {
        match self.storage_type {
            TodoStorageType::Csv => {
                let mut wtr = csv::Writer::from_path(&self.file_path)?;
                wtr.serialize(todo_list)?;
                Ok(())
            }
        }
    }

    pub fn load(&self) -> Result<TodoList, Box<dyn Error>> {
        match self.storage_type {
            TodoStorageType::Csv => {
                let mut rdr = csv::Reader::from_path(&self.file_path)?;
                let mut todos = Vec::new();
                for record in rdr.deserialize() {
                    let todo: Todo = record?;
                    todos.push(todo);
                }
                Ok(TodoList { todos })
            }
        }
    }
}
