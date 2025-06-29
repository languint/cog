#[cfg(test)]
mod func_exprs_test {
    use cog_parser::parser::{
        Parser,
        core::{expr::Expr, ops::BinaryOp, types::Types},
    };

    #[test]
    fn func_decl_base() {
        let input = "fn my_func() {}";
        let expected = vec![Expr::FunctionDeclaration {
            identifier: "my_func".into(),
            parameters: vec![],
            body: Box::new(Expr::Block(vec![])),
            return_type: None,
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn main_decl() {
        let input = "fn main() -> i32 {
            return 0;
        }";
        let expected = vec![Expr::FunctionDeclaration {
            identifier: "main".into(),
            parameters: vec![],
            body: Box::new(Expr::Block(vec![Expr::Return {
                value: Box::new(Expr::new_int_literal(0)),
            }])),
            return_type: Some(Types::I32),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }

    #[test]
    fn add_decl() {
        let input = "fn add(a: i32, b: i32) -> i32 {
            return a + b;
        }";
        let expected = vec![Expr::FunctionDeclaration {
            identifier: "main".into(),
            parameters: vec![],
            body: Box::new(Expr::Block(vec![Expr::Return {
                value: Box::new(Expr::Binary {
                    left: Box::new(Expr::new_identifier("a")),
                    operator: Box::new(BinaryOp::Add),
                    right: Box::new(Expr::new_identifier("b")),
                }),
            }])),
            return_type: Some(Types::I32),
        }];
        let mut parser = Parser::new(input.to_string()).expect("Failed to create parser");
        assert_eq!(parser.parse(), Ok(expected));
    }
}
