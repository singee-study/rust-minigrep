pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        assert_eq!(
            vec!["a"],
            search(
                "a",
                "\
a
b
c
d"
            )
        )
    }

    #[test]
    fn two_result() {
        assert_eq!(
            vec!["a", "ab"],
            search(
                "a",
                "\
a
ab
bc
cd"
            )
        )
    }
}
