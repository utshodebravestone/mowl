pub fn tokenize(src: &str) -> Vec<String> {
    src.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}

#[test]
fn test_tokenize() {
    let expectation = vec!["(", "+", "69", "420", ")"];
    let reality = tokenize("(+ 69 420)");
    assert_eq!(expectation, reality);
}
