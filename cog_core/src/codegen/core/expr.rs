use cog_parser::parser::core::{expr::Expr};

use crate::codegen::{CodeGen, errors::CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
        match expr {
            Expr::FunctionDeclaration {
                identifier,
                parameters,
                body,
                return_type,
            } => {
                self.compile_func_decl(identifier, parameters, body, return_type)?;
            }
            Expr::Return { value } => {
                self.build_return(value)?;
            }
            _ => todo!(),
        }
        Ok(())
    }
}
