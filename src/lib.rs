// Copyright 2025 itscheems
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
