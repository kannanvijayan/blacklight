mod code_block_builder;
mod shader_builder;

pub use self::{
  code_block_builder::CodeBlockBuilder,
  shader_builder::ShaderBuilder,
};

use crate::{
  api::{
    data_type::LiteralDataType,
    handle::ExprHandle,
  },
  model::{ ExpressionModel, LiteralExprModel },
};

/**
 * Create a new literal expression.
 */
pub fn literal<'cb, DT>(value: DT) -> ExprHandle<'cb, DT>
  where DT: LiteralDataType
{
  let literal_value = value.to_literal_data_value();
  let literal_expr_model = LiteralExprModel::new(literal_value);
  ExprHandle::new(Box::new(ExpressionModel::Literal(literal_expr_model)))
}
