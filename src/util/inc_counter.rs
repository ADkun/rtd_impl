/// 自增数生成器
pub struct IncCounter {
    count: u64,
}

impl IncCounter {
    /// 创建自增数生成器
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// 修改当前自增值
    pub fn set_count(&mut self, count: u64) {
        self.count = count;
    }

    /// 生成下一个自增数
    pub fn next(&mut self) -> u64 {
        self.count = self.count.wrapping_add(1);
        self.count
    }
}
