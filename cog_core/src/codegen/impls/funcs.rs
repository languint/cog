use cog_parser::parser::core::{expr::Expr, types::Types};

use crate::codegen::{CodeGen, errors::CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_func_decl(
        &mut self,
        name: &str,
        _args: &[Expr],
        body: &Expr,
        ret_type: &Option<Types>,
    ) -> Result<(), CodeGenError> {
        let i32_type = self.llvm_ctx.i32_type();
        let fn_type = i32_type.fn_type(&[], false); //TODO: Support var args and params later.

        let main_func = self.lvvm_module.add_function(name, fn_type, None);
        let entry = self.llvm_ctx.append_basic_block(main_func, "entry");
        self.llvm_builder.position_at_end(entry);
        self.main_func = Some(main_func);

        self.compile_body(body)?;

        Ok(())
    }

    pub fn compile_body(&mut self, body: &Expr) -> Result<(), CodeGenError> {
        match body {
            Expr::Block(exprs) => {
                for expr in exprs {
                    self.compile_expr(expr)?;
                }
            }
            _ => self.compile_expr(body)?,
        }

        Ok(())
    }

    pub fn build_return(&mut self, value: &Expr) -> Result<(), CodeGenError> {
        if let Expr::Literal(val) = value {
            if let cog_parser::parser::core::nodes::Nodes::Integer(v) = val {
                let i32_val = self.llvm_ctx.i32_type().const_int(*v as u64, true);
                let _ = self.llvm_builder.build_return(Some(&i32_val));
            } else {
                todo!();
            }
        } else {
            todo!();
        }

        Ok(())
    }
}
