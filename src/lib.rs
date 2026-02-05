// 一个简单的函数用于测试
pub fn split_words(s: &str) -> Vec<&str> {
	s.split_whitespace().collect()
}

// 一个计算函数用于演示
pub fn calculate_sum(numbers: &[i32]) -> i32 {
	numbers.iter().sum()
}

// 一个格式化函数用于演示
pub fn format_user(name: &str, age: u32, active: bool) -> String {
	format!("User: {}, Age: {}, Active: {}", name, age, active)
}

// 一个返回结构体的函数
#[derive(Debug, serde::Serialize)]
pub struct User {
	pub name: String,
	pub age: u32,
	pub active: bool,
}

impl User {
	pub fn new(name: &str, age: u32, active: bool) -> Self {
		Self {
			name: name.to_string(),
			age,
			active,
		}
	}
}
