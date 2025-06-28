#[cfg(test)]
mod math_expr_tests {
    use nht_parser::parser::{Parser, expr::Expr, nodes::Nodes, ops::BinaryOp};

    #[test]
    fn add_expr() {
        let input = "1 + 2";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(1)),
            right: Box::new(Expr::new_int_literal(2)),
            operator: Box::new(BinaryOp::Add),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn sub_expr() {
        let input = "3 - 4";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(3)),
            right: Box::new(Expr::new_int_literal(4)),
            operator: Box::new(BinaryOp::Subtract),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn mul_expr() {
        let input = "5 * 6";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(5)),
            right: Box::new(Expr::new_int_literal(6)),
            operator: Box::new(BinaryOp::Multiply),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn div_expr() {
        let input = "7 / 8";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::Literal(Nodes::Integer(7))),
            right: Box::new(Expr::Literal(Nodes::Integer(8))),
            operator: Box::new(BinaryOp::Divide),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn mod_expr() {
        let input = "9 % 10";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::new_int_literal(9)),
            right: Box::new(Expr::new_int_literal(10)),
            operator: Box::new(BinaryOp::Modulo),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn paren_expr() {
        let input = "(1 + 2) * 3";
        let expected = vec![Expr::Binary {
            left: Box::new(Expr::Binary {
                left: Box::new(Expr::new_int_literal(1)),
                operator: Box::new(BinaryOp::Add),
                right: Box::new(Expr::new_int_literal(2)),
            }),
            right: Box::new(Expr::new_int_literal(3)),
            operator: Box::new(BinaryOp::Multiply),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }
}
