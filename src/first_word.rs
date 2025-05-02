pub fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage(){
        let input = "howzit hoozle";
        let output = get_first_word(input);

        assert_eq!(output, "howzit");
    }
}
