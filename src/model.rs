//! 数据模型
//! - 数据结构
//! - 数据显示定义
//! - 序列化
//! - 反序列化

use std::collections::BTreeMap;
use std::fmt::Display;

use crate::util;
use serde::{Deserialize, Serialize};

/// 待办事项
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    /// 待办事项唯一id
    pub id: u64,

    /// 待办事项名称
    pub name: String,

    /// 该待办事项是否已完成
    pub completed: bool,

    /// 该待办事项是否已删除
    pub deleted: bool,

    /// 该待办事项创建的时间戳（秒）
    pub created_at: u64,

    /// 该待办事项完成的时间戳（秒）
    pub completed_at: Option<u64>,

    /// 该待办事项删除的时间戳（秒）
    pub deleted_at: Option<u64>,
}

impl Todo {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            completed: false,
            deleted: false,
            created_at: util::get_current_timestamp_secs(),
            completed_at: None,
            deleted_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(util::get_current_timestamp_secs());
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.completed_at = None;
    }

    pub fn delete(&mut self) {
        self.deleted = true;
        self.deleted_at = Some(util::get_current_timestamp_secs());
    }

    pub fn restore(&mut self) {
        self.deleted = false;
        self.deleted_at = None;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}

/// 待办事项列表
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub todos: BTreeMap<u64, Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: BTreeMap::new(),
        }
    }
}

#[test]
fn test_rand_u64() {
    println!("{}", rand::random::<u64>());
}
