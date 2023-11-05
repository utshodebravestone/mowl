pub fn tokenize(src: &str) -> Vec<String> {
    src.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}
