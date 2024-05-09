//! 模型映射
//! - ORM
//! - 文件读写

use crate::model::{Todo, TodoList};
use crate::util::path_exists;
use std::{collections::BTreeMap, error::Error};

/// 默认的Json文件相对路径
pub const DEFAULT_JSON_REL_FILE_PATH: &str = "todo.json";

/// 待办事项存储类型
pub enum TodoStorageType {
    Json,
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

        let result = Self {
            storage_type,
            file_path: file_path.to_string(),
        };

        if !path_exists(file_path) {
            result.save(&TodoList::new())?;
        }

        Ok(result)
    }

    pub fn save(&self, todo_list: &TodoList) -> Result<(), Box<dyn Error>> {
        match self.storage_type {
            TodoStorageType::Json => {
                let json_str = serde_json::to_string(todo_list)?;
                std::fs::write(&self.file_path, json_str)?;
                Ok(())
            }
        }
    }

    pub fn load(&self) -> Result<TodoList, Box<dyn Error>> {
        match self.storage_type {
            TodoStorageType::Json => {
                let json_str = std::fs::read_to_string(&self.file_path)?;
                let todos: TodoList = serde_json::from_str(&json_str)?;
                Ok(todos)
            }
        }
    }
}
