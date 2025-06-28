#[cfg(test)]
mod binary_exprs_tests {
    use nht_parser::parser::{
        Parser,
        core::{expr::Expr, ops::BinaryOp},
    };

    #[test]
    fn greater_expr() {
        let input = "1 > 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::Greater),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn less_expr() {
        let input = "1 < 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::Less),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn less_equal_expr() {
        let input = "1 <= 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::LessEqual),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn greater_equal_expr() {
        let input = "1 >= 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::GreaterEqual),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn equal_expr() {
        let input = "1 == 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::Equal),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn not_equal_expr() {
        let input = "1 != 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::NotEqual),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn and_expr() {
        let input = "1 && 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::And),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn or_expr() {
        let input = "1 || 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::Or),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }
}
