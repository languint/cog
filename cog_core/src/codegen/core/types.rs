use cog_parser::parser::core::types::Types;
use inkwell::{types, AddressSpace};

use crate::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn get_llvm_type(&self, parser_type: &Types) -> types::BasicTypeEnum<'ctx> {
        match parser_type {
            Types::I32 => self.llvm_ctx.i32_type().into(),
            Types::I64 => self.llvm_ctx.i64_type().into(),
            Types::F32 => self.llvm_ctx.f32_type().into(),
            Types::F64 => self.llvm_ctx.f64_type().into(),
            Types::Bool => self.llvm_ctx.bool_type().into(),
            Types::String => self.llvm_ctx.ptr_type(AddressSpace::default()).into(),
        }
    }
}