#[cfg(test)]
mod if_exprs_tests {
    use nht_parser::parser::{Parser, core::expr::Expr};

    #[test]
    fn if_exprs() {
        let input = "if true { 1 } else { 2 }";
        let expected = vec![Expr::IfElse {
            condition: Box::new(Expr::new_boolean_literal(true)),
            then_branch: Box::new(Expr::Block(vec![Expr::new_int_literal(1)])),
            else_branch: Some(Box::new(Expr::Block(vec![Expr::new_int_literal(2)]))),
        }];

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");

        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn if_exprs_without_else() {
        let input = "if true { 1 }";
        let expected = vec![Expr::IfElse {
            condition: Box::new(Expr::new_boolean_literal(true)),
            then_branch: Box::new(Expr::Block(vec![Expr::new_int_literal(1)])),
            else_branch: None,
        }];

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");

        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn if_exprs_with_nested_if() {
        let input = "if true { if false { 1 } else { 2 } } else { 3 }";
        let expected = vec![Expr::IfElse {
            condition: Box::new(Expr::new_boolean_literal(true)),
            then_branch: Box::new(Expr::Block(vec![Expr::IfElse {
                condition: Box::new(Expr::new_boolean_literal(false)),
                then_branch: Box::new(Expr::Block(vec![Expr::new_int_literal(1)])),
                else_branch: Some(Box::new(Expr::Block(vec![Expr::new_int_literal(2)]))),
            }])),
            else_branch: Some(Box::new(Expr::Block(vec![Expr::new_int_literal(3)]))),
        }];

        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");

        assert_eq!(parser.parse(), Ok(expected));
    }
}
