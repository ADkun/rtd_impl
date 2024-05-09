//! 业务逻辑

use std::error::Error;

use crate::model::{Todo, TodoList};
use crate::storage::{TodoStorage, TodoStorageType, DEFAULT_JSON_REL_FILE_PATH};
use crate::util::IncCounter;

/// Todo的状态（用于查询）
#[derive(Debug)]
pub enum TodoStatus {
    All,
    Valid,
    Completed,
    Uncompleted,
    Deleted,
}

/// Todo服务层
pub struct TodoService {
    list: TodoList,
    storage: TodoStorage,
    inc_counter: IncCounter,
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
                eprintln!("加载todo列表失败: {}", e);
                return Err(e);
            }
        }

        self.inc_counter.set_count(
            self.list
                .todos
                .iter()
                .map(|(_, todo)| todo.id)
                .max()
                .unwrap_or(0),
        );

        self.init_done = true;

        Ok(())
    }

    pub fn add_todo(&mut self, name: &str) {
        assert!(self.init_done);

        let id = self.inc_counter.next();
        let todo = Todo::new(id, name);
        self.list.todos.insert(id, todo);

        println!("{} 已添加 {}", id, name);
    }

    pub fn show_todo_list(&self, status: TodoStatus) {
        assert!(self.init_done);

        for (_, todo) in self.list.todos.iter() {
            match status {
                TodoStatus::All => {
                    println!("{}", todo);
                }
                TodoStatus::Completed => {
                    if todo.is_completed() {
                        println!("{}", todo);
                    }
                }
                TodoStatus::Uncompleted => {
                    if !todo.is_completed() {
                        println!("{}", todo);
                    }
                }
                TodoStatus::Deleted => {
                    if todo.is_deleted() {
                        println!("{}", todo);
                    }
                }
                TodoStatus::Valid => {
                    if !todo.is_deleted() {
                        println!("{}", todo);
                    }
                }
            }
        }
    }

    pub fn complete_todo(&mut self, id: u64) {
        assert!(self.init_done);

        if let Some(todo) = self.list.todos.get_mut(&id) {
            todo.complete();
        }

        println!("{} 已完成", id);
    }

    pub fn uncomplete_todo(&mut self, id: u64) {
        assert!(self.init_done);

        if let Some(todo) = self.list.todos.get_mut(&id) {
            todo.uncomplete();
        }

        println!("{} 已取消完成", id);
    }

    pub fn delete_todo(&mut self, id: u64) {
        assert!(self.init_done);

        if let Some(todo) = self.list.todos.get_mut(&id) {
            todo.delete();
        }

        println!("{} 已标记删除", id);
    }

    pub fn restore_todo(&mut self, id: u64) {
        assert!(self.init_done);

        if let Some(todo) = self.list.todos.get_mut(&id) {
            todo.restore();
        }

        println!("{} 已恢复", id);
    }

    pub fn destroy_todo(&mut self, id: u64) {
        assert!(self.init_done);

        if let Some(key) = self.list.todos.remove(&id) {
            println!("{} 已删除", key);
        } else {
            eprintln!("{} 不存在", id);
        }
    }

    pub fn destroy_deleted_todo(&mut self) {
        assert!(self.init_done);

        self.list.todos.retain(|_, todo| !todo.is_deleted());

        println!("已清理已删除的 todo");
    }

    pub fn clear_todo(&mut self) {
        assert!(self.init_done);

        self.list.todos.clear();

        println!("已清空 todo 列表");
    }
}

impl Default for TodoService {
    /// 默认构造器
    fn default() -> Self {
        Self {
            list: TodoList::new(),
            storage: TodoStorage::new(TodoStorageType::Json, DEFAULT_JSON_REL_FILE_PATH).unwrap(),
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
