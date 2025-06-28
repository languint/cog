#[cfg(test)]
mod assign_exprs {
    use nht_parser::parser::{Parser, expr::Expr, types::Types};

    #[test]
    fn let_no_type() {
        let input = "let x = 1";
        let expected = vec![Expr::Declaration {
            identifier: "x".to_owned(),
            var_type: None,
            value: Box::new(Expr::new_int_literal(1)),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn let_with_type() {
        let input = "let x: i32 = 1";
        let expected = vec![Expr::Declaration {
            identifier: "x".to_owned(),
            var_type: Some(Types::I32),
            value: Box::new(Expr::new_int_literal(1)),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn malformed_let_missing_type() {
        let input = "let x: = 1";

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert!(parser.parse().is_err());
    }

    #[test]
    fn malformed_let_missing_value() {
        let input = "let x: i32";

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert!(parser.parse().is_err());
    }

    #[test]
    fn malformed_let_missing_identifier() {
        let input = "let : i32 = 1";

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert!(parser.parse().is_err());
    }

    #[test]
    fn malformed_let_missing_colon() {
        let input = "let x i32 = 1";

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert!(parser.parse().is_err());
    }

    #[test]
    fn malformed_let_missing_equals() {
        let input = "let x i32 1";

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert!(parser.parse().is_err());
    }
}
