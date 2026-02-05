use example::{User, calculate_sum, format_user, split_words};

#[test]
fn test_split_words() {
	let words = split_words("hello from the other side");
	insta::assert_yaml_snapshot!(words);
}

#[test]
fn test_calculate_sum() {
	let numbers = vec![1, 2, 3, 4, 5, 6];
	let sum = calculate_sum(&numbers);
	insta::assert_yaml_snapshot!(sum);
}

#[test]
fn test_format_user() {
	let user_str = format_user("Alice", 30, true);
	insta::assert_snapshot!(user_str);
}

#[test]
fn test_user_struct() {
	let user = User::new("Bob", 25, false);
	insta::assert_yaml_snapshot!(user);
}

#[test]
fn test_debug_snapshot() {
	let data = vec![("name", "Charlie"), ("age", "35"), ("city", "Beijing")];
	insta::assert_debug_snapshot!(data);
}
