use crate::tokenizer::tokenize;

#[test]
fn test_tokenizer() {
    let expectation = vec!["(", "+", "69", "420", ")"];
    let reality = tokenize("(+ 69 420)");
    assert_eq!(expectation, reality);
}
