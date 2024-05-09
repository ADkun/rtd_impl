//! 业务逻辑

use std::error::Error;

use crate::model::{Todo, TodoList};
use crate::storage::{TodoStorage, TodoStorageType, DEFAULT_CSV_REL_FILE_PATH};
use crate::util::IncCounter;

/// Todo的状态（用于查询）
#[derive(Debug)]
enum TodoStatus {
    All,
    Completed,
    Uncompleted,
    Deleted,
}

/// Todo服务层
pub struct TodoService {
    pub list: TodoList,
    pub storage: TodoStorage,
    pub inc_counter: IncCounter,
    init_done: bool,
}

impl TodoService {
    pub fn new() -> Self {
        TodoService::default()
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        match self.storage.load() {
            Ok(todo_list) => self.list = todo_list,
            Err(e) => {
                eprintln!("Error loading todo list: {}", e);
                return Err(e);
            }
        }

        self.inc_counter.set_count(
            self.list
                .todos
                .iter()
                .map(|todo| todo.id)
                .max()
                .unwrap_or(0),
        );

        Ok(())
    }

    pub fn add_todo(&mut self, name: &str) -> Result<u32, Box<dyn Error>> {
        assert!(self.init_done);
        todo!()
    }
}

impl Default for TodoService {
    /// 默认构造器
    fn default() -> Self {
        Self {
            list: TodoList::new(),
            storage: TodoStorage::new(TodoStorageType::Csv, DEFAULT_CSV_REL_FILE_PATH).unwrap(),
            inc_counter: IncCounter::new(),
            init_done: false,
        }
    }
}

impl Drop for TodoService {
    /// 自动保存
    fn drop(&mut self) {
        self.storage.save(&self.list).unwrap();
    }
}
