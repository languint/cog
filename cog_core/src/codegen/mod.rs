use std::collections::HashMap;

use cog_parser::parser::core::expr::Expr;
use cog_parser::parser::core::types::Types;
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::BasicTypeEnum,
    values::{FunctionValue, PointerValue},
};

use crate::codegen::errors::CodeGenError;
pub mod errors;

pub struct CodeGen<'ctx> {
    pub llvm_ctx: &'ctx Context,
    pub lvvm_module: Module<'ctx>,
    pub llvm_builder: Builder<'ctx>,
    variables: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
    main_func: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();

        Self {
            lvvm_module: module,
            llvm_builder: builder,
            llvm_ctx: context,
            variables: HashMap::new(),
            main_func: None,
        }
    }

    pub fn compile(&mut self, statements: &[Expr]) -> Result<(), CodeGenError> {
        for stmt in statements {
            self.compile_expr(stmt)?;
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CodeGenError> {
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

    fn compile_func_decl(
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

    fn compile_body(&mut self, body: &Expr) -> Result<(), CodeGenError> {
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

    fn build_return(&mut self, value: &Expr) -> Result<(), CodeGenError> {
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
