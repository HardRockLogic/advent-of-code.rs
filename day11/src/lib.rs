mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use parser::list;

    #[test]
    fn test_list() {
        assert_eq!(list("Starting items: 12, 3, 5\n"), Ok(("", vec![12, 3, 5])));
    }
}
