pub fn example() -> String {
    format!("hello, world!")
}

#[test]
fn test_example() {
    assert_eq!(example(), String::from("hello, world!"));
}
