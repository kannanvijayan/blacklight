use std::{
  marker::PhantomData,
  ops,
};
use crate::{
  api::{
    data_type::{
      ExprDataType,
      ExprNumericDataType,
      ExprScalarNumericDataType,
      ExprVectorNumericDataType,
      Struct,
      StructMappedDataType,
    },
    handle::LvalueHandle,
  },
  data_type::HostShareableDataType,
  model::{
    BinOp,
    BinOpExprModel,
    CmpOp,
    CmpOpExprModel,
    ExpressionModel,
    IdentifierModel,
    LvalueModel,
    StructFieldReadModel,
    VecConstructorExprModel,
  }
};

/**
 * A typed, lifetime-aware handle to an expression within a shader.
 */
pub struct ExprHandle<'cb, DT: ExprDataType> {
  pub(crate) model: Box<ExpressionModel>,
  _phantom: PhantomData<&'cb DT>,
}
impl<'cb, DT: ExprDataType> ExprHandle<'cb, DT> {
  /** Create a new expression handle for the given expression model. */
  pub(crate) fn new(model: Box<ExpressionModel>) -> Self {
    ExprHandle { model, _phantom: PhantomData }
  }

  /** Build a comparison expression. */
  pub fn cmp(&self, other: &Self, cmp_op: CmpOp) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    let cmp_op_expr_model =
      CmpOpExprModel::new(self.model.clone(), other.model.clone(), cmp_op);
    let model = ExpressionModel::CmpOp(cmp_op_expr_model);
    ExprHandle::new(Box::new(model))
  }

  /** Build an equality `==` expression. */
  pub fn eq(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Eq)
  }

  /** Build an non-equality `!=` expression. */
  pub fn ne(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Ne)
  }

  /** Build a less-than `<` expression. */
  pub fn lt(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Lt)
  }

  /** Build a less-than-or-equal-to `<=` expression. */
  pub fn le(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Le)
  }

  /** Build a greater-than `>` expression. */
  pub fn gt(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Gt)
  }

  /** Build a greater-than-or-equal-to `>=` expression. */
  pub fn ge(&self, other: &Self) -> ExprHandle<'cb, bool>
    where DT: ExprNumericDataType
  {
    self.cmp(other, CmpOp::Ge)
  }

  /** Cast scalar to vector. */
  pub fn to_vec<VDT>(&self) -> ExprHandle<'cb, VDT>
    where DT: ExprScalarNumericDataType,
          VDT: ExprVectorNumericDataType,
  {
    let vec_constructor_model = VecConstructorExprModel::new(
      VDT::DIMS,
      DT::repr(),
      vec![self.model.clone()],
    );
    let model = ExpressionModel::VecConstructor(vec_constructor_model);
    ExprHandle::new(Box::new(model))
  }
}
impl<'cb, DT: ExprDataType> Clone for ExprHandle<'cb, DT> {
  fn clone(&self) -> Self {
    ExprHandle { model: self.model.clone(), _phantom: PhantomData }
  }
}

// Generic binop helper.
fn make_binop<'cb, LHS, RHS, RES>(
  lhs: ExprHandle<'cb, LHS>,
  rhs: ExprHandle<'cb, RHS>,
  binop: BinOp
) -> ExprHandle<'cb, RES>
  where LHS: ExprNumericDataType,
        RHS: ExprNumericDataType,
        RES: ExprNumericDataType
{
  let binop_expr_model = BinOpExprModel::new(lhs.model, rhs.model, binop);
  let expr_model = ExpressionModel::BinOp(binop_expr_model);
  ExprHandle::new(Box::new(expr_model))
}

// Macro to generate binary operator implementations between different types.
macro_rules! impl_binop {
  (
    $class:ident,
    $lhs:ty,
    $rhs:ty,
    $res:ty,
    $op:ident,
    $binop:expr
  ) => {
    impl<'cb> ops::$class<ExprHandle<'cb, $rhs>> for ExprHandle<'cb, $lhs> {
      type Output = ExprHandle<'cb, $res>;
      fn $op(self, other: ExprHandle<'cb, $rhs>) -> Self::Output {
        make_binop(self, other, $binop)
      }
    }
  };
}

// Bulk-implementation of same-type binary operators.
macro_rules! impl_binop_sametype_arith_bulk {
  ($($type:ty),*) => {
    $(
      impl_binop!(Add, $type, $type, $type, add, BinOp::Add);
      impl_binop!(Sub, $type, $type, $type, sub, BinOp::Sub);
      impl_binop!(Mul, $type, $type, $type, mul, BinOp::Mul);
      impl_binop!(Div, $type, $type, $type, div, BinOp::Div);
      impl_binop!(Rem, $type, $type, $type, rem, BinOp::Rem);
    )*
  };
}

// Bulk-implementation of same-type bit operators.
macro_rules! impl_binop_sametype_bit_bulk {
  ($($type:ty),*) => {
    $(
      impl_binop!(BitOr, $type, $type, $type, bitor, BinOp::BitOr);
      impl_binop!(BitAnd, $type, $type, $type, bitand, BinOp::BitAnd);
      impl_binop!(BitXor, $type, $type, $type, bitxor, BinOp::BitXor);
    )*
  };
}

// Bulk-implementation of vector-scalar arith operators.
macro_rules! impl_binop_vec_scalar_arith_bulk {
  ($(($vecfield:ty, $scalar:ty)),*) => {
    $(
      impl_binop!(Add, $vecfield, $scalar, $vecfield, add, BinOp::Add);
      impl_binop!(Sub, $vecfield, $scalar, $vecfield, sub, BinOp::Sub);
      impl_binop!(Mul, $vecfield, $scalar, $vecfield, mul, BinOp::Mul);
      impl_binop!(Div, $vecfield, $scalar, $vecfield, div, BinOp::Div);
      impl_binop!(Rem, $vecfield, $scalar, $vecfield, rem, BinOp::Rem);
    )*
  };
}

// Bulk-implementation of vector-scalar bit operators.
macro_rules! impl_binop_vec_scalar_bit_bulk {
  ($(($vecfield:ty, $scalar:ty)),*) => {
    $(
      impl_binop!(BitOr, $vecfield, $scalar, $vecfield, bitor, BinOp::BitOr);
      impl_binop!(BitAnd, $vecfield, $scalar, $vecfield, bitand, BinOp::BitAnd);
      impl_binop!(BitXor, $vecfield, $scalar, $vecfield, bitxor, BinOp::BitXor);
    )*
  };
}

// Bulk-implementation of shift operators.
macro_rules! impl_binop_vec_scalar_shift_bulk {
  ($(($lhs:ty, $rhs:ty)),*) => {
    $(
      impl_binop!(Shl, $lhs, $rhs, $lhs, shl, BinOp::Shl);
      impl_binop!(Shr, $lhs, $rhs, $lhs, shr, BinOp::Shr);
    )*
  };
}

impl_binop_sametype_arith_bulk!(u32, [u32; 2], [u32; 3], [u32; 4]);
impl_binop_sametype_arith_bulk!(i32, [i32; 2], [i32; 3], [i32; 4]);
impl_binop_sametype_arith_bulk!(f32, [f32; 2], [f32; 3], [f32; 4]);

impl_binop_sametype_bit_bulk!(u32, [u32; 2], [u32; 3], [u32; 4]);
impl_binop_sametype_bit_bulk!(i32, [i32; 2], [i32; 3], [i32; 4]);

impl_binop_vec_scalar_arith_bulk!(
  ([u32; 2], u32),
  ([u32; 3], u32),
  ([u32; 4], u32),

  ([i32; 2], i32),
  ([i32; 3], i32),
  ([i32; 4], i32),

  ([f32; 2], f32),
  ([f32; 3], f32),
  ([f32; 4], f32),

  (u32, [u32; 2]),
  (u32, [u32; 3]),
  (u32, [u32; 4]),

  (i32, [i32; 2]),
  (i32, [i32; 3]),
  (i32, [i32; 4]),

  (f32, [f32; 2]),
  (f32, [f32; 3]),
  (f32, [f32; 4])
);

impl_binop_vec_scalar_bit_bulk!(
  ([u32; 2], u32),
  ([u32; 3], u32),
  ([u32; 4], u32),

  ([i32; 2], i32),
  ([i32; 3], i32),
  ([i32; 4], i32),

  (u32, [u32; 2]),
  (u32, [u32; 3]),
  (u32, [u32; 4]),

  (i32, [i32; 2]),
  (i32, [i32; 3]),
  (i32, [i32; 4])
);

impl_binop_vec_scalar_shift_bulk!(
  (u32, u32),
  ([u32; 2], u32),
  ([u32; 3], u32),
  ([u32; 4], u32),

  ([u32; 2], [u32; 2]),
  ([u32; 3], [u32; 3]),
  ([u32; 4], [u32; 4]),

  (i32, u32),
  ([i32; 2], u32),
  ([i32; 3], u32),
  ([i32; 4], u32),

  ([i32; 2], [u32; 2]),
  ([i32; 3], [u32; 3]),
  ([i32; 4], [u32; 4]),

  (u32, [u32; 2]),
  (u32, [u32; 3]),
  (u32, [u32; 4]),

  (i32, [u32; 2]),
  (i32, [u32; 3]),
  (i32, [u32; 4])
);

// Methods available on struct-type expressions.
impl<'cb, DT> ExprHandle<'cb, Struct<DT>>
  where DT: StructMappedDataType
{
  /** Read a field from the struct. */
  pub fn read<FT>(&self, name: &str) -> ExprHandle<'cb, FT>
    where FT: ExprDataType
  {
    let struct_repr = Struct::<DT>::make_struct_repr();
    let maybe_field = struct_repr.get_field(name);
    let field = match maybe_field {
      Some(field) => field,
      None => panic!("Field '{}' not found in struct", name),
    };
    let read_repr = FT::repr();
    if field.data_type() != &read_repr {
      panic!("Field '{}' has type {:?}, expected {:?}", name, field.data_type(), FT::repr());
    }

    let ident = IdentifierModel::new(name);
    let field_read_model = StructFieldReadModel::new(
      self.model.clone(),
      ident,
      read_repr,
    );
    let expr_model = ExpressionModel::StructFieldRead(field_read_model);
    ExprHandle::new(Box::new(expr_model))
  }

  /** Reference a field from the struct. */
  pub fn get<FT>(&self, name: &str) -> ExprHandle<'cb, FT>
    where FT: HostShareableDataType
  {
    let struct_repr = Struct::<DT>::make_struct_repr();
    let maybe_field = struct_repr.get_field(name);
    let field = match maybe_field {
      Some(field) => field,
      None => panic!("Field '{}' not found in struct", name),
    };
    let read_repr = FT::repr();
    if field.data_type() != &read_repr {
      panic!("Field '{}' has type {:?}, expected {:?}", name, field.data_type(), FT::repr());
    }

    let ident = IdentifierModel::new(name);
    let field_read_model = StructFieldReadModel::new(
      self.model.clone(),
      ident,
      read_repr,
    );
    let expr_model = ExpressionModel::StructFieldRead(field_read_model);
    ExprHandle::new(Box::new(expr_model))
  }

  /** Reference a field from the struct. */
  pub fn field<FT>(&self, name: &str) -> LvalueHandle<'cb, FT>
    where FT: HostShareableDataType
  {
    let struct_repr = Struct::<DT>::make_struct_repr();
    let maybe_field = struct_repr.get_field(name);
    let field = match maybe_field {
      Some(field) => field,
      None => panic!("Field '{}' not found in struct", name),
    };
    let read_repr = FT::repr();
    if field.data_type() != &read_repr {
      panic!("Field '{}' has type {:?}, expected {:?}", name, field.data_type(), FT::repr());
    }

    let ident = IdentifierModel::new(name);
    LvalueHandle::new(
      LvalueModel::new_struct_field(self.model.clone(), ident, read_repr)
    )
  }
}
