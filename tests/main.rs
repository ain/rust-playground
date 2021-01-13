#[cfg(test)]
mod tests {

    #[test]
    fn ascii_alphabet() {
        let expectation = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        assert_eq!(expectation, functions::ascii_alphabet());
    }
}
