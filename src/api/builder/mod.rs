mod code_block_builder;
mod shader_builder;

use std::vec;

pub use self::{
  code_block_builder::CodeBlockBuilder,
  shader_builder::ShaderBuilder,
};

use crate::{
  api::{
    data_type::{ ExprVectorNumericDataType, LiteralDataType },
    handle::ExprHandle,
  },
  model::{ ExpressionModel, LiteralExprModel, VecConstructorExprModel },
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

pub trait VecTransformTo<'cb, DT: ExprVectorNumericDataType> {
  fn make_handle(self) -> ExprHandle<'cb, DT>;
}

/**
 * Create a new builtin vec constructor expression.
 */
pub fn mkvec<'cb, DT, DT2>(v: DT2) -> ExprHandle<'cb, DT>
  where
    DT: ExprVectorNumericDataType,
    DT2: VecTransformTo<'cb, DT>,
{
  v.make_handle()
}

fn make_handle_impl<'cb, DT>(
  model_vec: Vec<Box<ExpressionModel>>,
) -> ExprHandle<'cb, DT>
  where DT: ExprVectorNumericDataType,
{
  let vec_constructor_model = VecConstructorExprModel::new(
    DT::DIMS,
    DT::repr(),
    model_vec,
  );
  let model = ExpressionModel::VecConstructor(vec_constructor_model);
  ExprHandle::new(Box::new(model))
}

// Macro to implement VecTransformTo for a single scalar or vector rust type.
macro_rules! impl_vec_transform_1_to {
  ($target_ty:ty, $dims:expr, $($scalar_or_vector_ty:ty),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty> for $scalar_or_vector_ty {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![literal(self).model])
        }
      }
    )*
  };
}

impl_vec_transform_1_to!([u32; 2], 2, u32, i32, f32);
impl_vec_transform_1_to!([u32; 3], 3, u32, i32, f32);
impl_vec_transform_1_to!([u32; 4], 4, u32, i32, f32);

impl_vec_transform_1_to!([i32; 2], 2, u32, i32, f32);
impl_vec_transform_1_to!([i32; 3], 3, u32, i32, f32);
impl_vec_transform_1_to!([i32; 4], 4, u32, i32, f32);

impl_vec_transform_1_to!([f32; 2], 2, u32, i32, f32);
impl_vec_transform_1_to!([f32; 3], 3, u32, i32, f32);
impl_vec_transform_1_to!([f32; 4], 4, u32, i32, f32);


// Macro to implement VecTransformTo for a single scalar or vector handle type.
macro_rules! impl_handle_vec_transform_1_to {
  ($target_ty:ty, $dims:expr, $($scalar_or_vector_ty:ty),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty>
      for ExprHandle<'cb, $scalar_or_vector_ty>
      {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![self.model])
        }
      }
    )*
  };
}

impl_handle_vec_transform_1_to!([u32; 2], 2, u32, i32, f32);
impl_handle_vec_transform_1_to!([u32; 3], 3, u32, i32, f32);
impl_handle_vec_transform_1_to!([u32; 4], 4, u32, i32, f32);

impl_handle_vec_transform_1_to!([i32; 2], 2, u32, i32, f32);
impl_handle_vec_transform_1_to!([i32; 3], 3, u32, i32, f32);
impl_handle_vec_transform_1_to!([i32; 4], 4, u32, i32, f32);

impl_handle_vec_transform_1_to!([f32; 2], 2, u32, i32, f32);
impl_handle_vec_transform_1_to!([f32; 3], 3, u32, i32, f32);
impl_handle_vec_transform_1_to!([f32; 4], 4, u32, i32, f32);


// Macro to implement VecTransformTo for two scalar or vector rust types.
macro_rules! impl_vec_transform_2_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty,$ty1:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty> for ($ty0, $ty1) {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![literal(self.0).model, literal(self.1).model])
        }
      }
    )*
  };
}

impl_vec_transform_2_to!([u32; 2], 2, (u32, u32), (i32, i32), (f32, f32));
impl_vec_transform_2_to!([i32; 2], 2, (u32, u32), (i32, i32), (f32, f32));
impl_vec_transform_2_to!([f32; 2], 2, (u32, u32), (i32, i32), (f32, f32));


impl_vec_transform_2_to!([u32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_vec_transform_2_to!([u32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));

impl_vec_transform_2_to!([i32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_vec_transform_2_to!([i32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));

impl_vec_transform_2_to!([f32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_vec_transform_2_to!([f32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));


impl_vec_transform_2_to!([u32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_vec_transform_2_to!([u32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_vec_transform_2_to!([u32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));

impl_vec_transform_2_to!([i32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_vec_transform_2_to!([i32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_vec_transform_2_to!([i32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));

impl_vec_transform_2_to!([f32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_vec_transform_2_to!([f32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_vec_transform_2_to!([f32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));


// Macro to implement VecTransformTo for two scalar or vector handle types.
macro_rules! impl_handle_vec_transform_2_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty, $ty1:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty>
      for (ExprHandle<'cb, $ty0>, ExprHandle<'cb, $ty1>)
      {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![self.0.model, self.1.model])
        }
      }
    )*
  };
}

impl_handle_vec_transform_2_to!([u32; 2], 2, (u32, u32), (i32, i32), (f32, f32));
impl_handle_vec_transform_2_to!([i32; 2], 2, (u32, u32), (i32, i32), (f32, f32));
impl_handle_vec_transform_2_to!([f32; 2], 2, (u32, u32), (i32, i32), (f32, f32));


impl_handle_vec_transform_2_to!([u32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_handle_vec_transform_2_to!([u32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));

impl_handle_vec_transform_2_to!([i32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_handle_vec_transform_2_to!([i32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));

impl_handle_vec_transform_2_to!([f32; 3], 3, (u32, [u32; 2]), (i32, [i32; 2]), (f32, [f32; 2]));
impl_handle_vec_transform_2_to!([f32; 3], 3, ([u32; 2], u32), ([i32; 2], i32), ([f32; 2], f32));


impl_handle_vec_transform_2_to!([u32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_handle_vec_transform_2_to!([u32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_handle_vec_transform_2_to!([u32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));

impl_handle_vec_transform_2_to!([i32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_handle_vec_transform_2_to!([i32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_handle_vec_transform_2_to!([i32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));

impl_handle_vec_transform_2_to!([f32; 4], 4, (u32, [u32; 3]), (i32, [i32; 3]), (f32, [f32; 3]));
impl_handle_vec_transform_2_to!([f32; 4], 4, ([u32; 3], u32), ([i32; 3], i32), ([f32; 3], f32));
impl_handle_vec_transform_2_to!([f32; 4], 4, ([u32; 2], [u32; 2]), ([i32; 2], [i32; 2]), ([f32; 2], [f32; 2]));


// Macro to implement VecTransformTo for two scalar or vector rust types.
macro_rules! impl_vec_transform_3_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty,$ty1:ty,$ty2:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty> for ($ty0, $ty1, $ty2) {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![
            literal(self.0).model,
            literal(self.1).model,
            literal(self.2).model
          ])
        }
      }
    )*
  };
}

impl_vec_transform_3_to!([u32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));
impl_vec_transform_3_to!([i32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));
impl_vec_transform_3_to!([f32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));


impl_vec_transform_3_to!([u32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_vec_transform_3_to!([u32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_vec_transform_3_to!([u32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

impl_vec_transform_3_to!([i32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_vec_transform_3_to!([i32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_vec_transform_3_to!([i32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

impl_vec_transform_3_to!([f32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_vec_transform_3_to!([f32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_vec_transform_3_to!([f32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

// Macro to implement VecTransformTo for two scalar or vector handle types.
macro_rules! impl_handle_vec_transform_3_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty, $ty1:ty, $ty2:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty>
      for (ExprHandle<'cb, $ty0>, ExprHandle<'cb, $ty1>, ExprHandle<'cb, $ty2>)
      {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![self.0.model, self.1.model, self.2.model])
        }
      }
    )*
  };
}

impl_handle_vec_transform_3_to!([u32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));
impl_handle_vec_transform_3_to!([i32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));
impl_handle_vec_transform_3_to!([f32; 3], 3, (u32, u32, u32), (i32, i32, i32), (f32, f32, f32));


impl_handle_vec_transform_3_to!([u32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_handle_vec_transform_3_to!([u32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_handle_vec_transform_3_to!([u32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

impl_handle_vec_transform_3_to!([i32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_handle_vec_transform_3_to!([i32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_handle_vec_transform_3_to!([i32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

impl_handle_vec_transform_3_to!([f32; 4], 4, (u32, u32, [u32; 2]), (i32, i32, [i32; 2]), (f32, f32, [f32; 2]));
impl_handle_vec_transform_3_to!([f32; 4], 4, (u32, [u32; 2], u32), (i32, [i32; 2], i32), (f32, [f32; 2], f32));
impl_handle_vec_transform_3_to!([f32; 4], 4, ([u32; 2], u32, u32), ([i32; 2], i32, i32), ([f32; 2], f32, f32));

// Macro to implement VecTransformTo for two scalar or vector rust types.
macro_rules! impl_vec_transform_4_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty,$ty1:ty,$ty2:ty,$ty3:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty> for ($ty0, $ty1, $ty2, $ty3) {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![
            literal(self.0).model,
            literal(self.1).model,
            literal(self.2).model,
            literal(self.3).model
          ])
        }
      }
    )*
  };
}

impl_vec_transform_4_to!([u32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));
impl_vec_transform_4_to!([i32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));
impl_vec_transform_4_to!([f32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));

// Macro to implement VecTransformTo for two scalar or vector handle types.
macro_rules! impl_handle_vec_transform_4_to {
  ($target_ty:ty, $dims:expr, $(($ty0:ty, $ty1:ty, $ty2:ty, $ty3:ty)),*) => {
    $(
      impl<'cb> VecTransformTo<'cb, $target_ty>
      for (
        ExprHandle<'cb, $ty0>,
        ExprHandle<'cb, $ty1>,
        ExprHandle<'cb, $ty2>,
        ExprHandle<'cb, $ty3>
      )
      {
        fn make_handle(self) -> ExprHandle<'cb, $target_ty> {
          make_handle_impl(vec![
            self.0.model,
            self.1.model,
            self.2.model,
            self.3.model
          ])
        }
      }
    )*
  };
}

impl_handle_vec_transform_4_to!([u32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));
impl_handle_vec_transform_4_to!([i32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));
impl_handle_vec_transform_4_to!([f32; 4], 4, (u32, u32, u32, u32), (i32, i32, i32, i32), (f32, f32, f32, f32));
