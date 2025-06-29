use std::collections::HashMap;

use cog_parser::parser::core::expr::Expr;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::BasicTypeEnum,
    values::{FunctionValue, PointerValue},
};

use crate::codegen::errors::CodeGenError;
pub mod core;
pub mod errors;
pub mod impls;

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
}
