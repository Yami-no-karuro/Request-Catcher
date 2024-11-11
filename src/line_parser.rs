pub fn get_all(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn get_parts(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}
