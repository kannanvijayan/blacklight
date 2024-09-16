use crate::{
  api::{
    data_type::{ ExprDataType, DataTypeRepr },
    handle::ExprHandle,
  },
  model::{ ExpressionModel, IdentifierModel, IdentifierExprModel },
};

/**
 * Trait for tuples of shader argument data types.
 */
pub trait ArgTupleDataType {
  /** The number of elements in the tuple. */
  const LENGTH: usize;
}
impl ArgTupleDataType for () {
  const LENGTH: usize = 0;
}
impl<T> ArgTupleDataType for (T,)
  where T: ExprDataType
{
  const LENGTH: usize = 1;
}
impl<T1, T2> ArgTupleDataType for (T1, T2)
  where T1: ExprDataType, T2: ExprDataType
{
  const LENGTH: usize = 2;
}
impl<T1, T2, T3> ArgTupleDataType for (T1, T2, T3)
  where T1: ExprDataType, T2: ExprDataType, T3: ExprDataType
{
  const LENGTH: usize = 3;
}
impl<T1, T2, T3, T4> ArgTupleDataType for (T1, T2, T3, T4)
  where T1: ExprDataType, T2: ExprDataType, T3: ExprDataType, T4: ExprDataType
{
  const LENGTH: usize = 4;
}

/**
 * Mapping trait to map a static tuple of argument rust types to
 * a lifetime-bound tuple of expression handles for those types.
 */
pub trait ArgTupleHandleMap<'a> {
  type HandleTuple: 'a;
  type NameTuple: 'a;

  fn make_handle_tuple(names: &Self::NameTuple) -> Self::HandleTuple;
  fn make_names_vector(names: &Self::NameTuple) -> Vec<IdentifierModel>;
  fn make_types_vector() -> Vec<DataTypeRepr>;
  fn visit_argument_handles<V>(handles: Self::HandleTuple, visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>;
}

/**
 * A visitor for argument expression handle tuples.
 */
pub trait ArgTupleHandleVisitor<'a> {
  fn visit_arg<ARG>(&mut self, handles: ExprHandle<'a, ARG>)
    where ARG: ExprDataType;
}

//////////////////////////////////////////////////////////////////////
/// Implementations for ArgTupleHandleMap for tuples of argument types.

impl<'a> ArgTupleHandleMap<'a> for () {
  type HandleTuple = ();
  type NameTuple = ();

  fn make_handle_tuple(_names: &Self::NameTuple) -> Self::HandleTuple {
    ()
  }
  fn make_names_vector(_names: &Self::NameTuple) -> Vec<IdentifierModel> {
    Vec::new()
  }
  fn make_types_vector() -> Vec<DataTypeRepr> {
    Vec::new()
  }
  fn visit_argument_handles<V>(_handles: Self::HandleTuple, _visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>
  {}
}


fn make_identifier_expr<'a, T>(name: &'static str) -> ExprHandle<'a, T>
  where T: ExprDataType
{
  let identifier_model = IdentifierModel::new(name);
  let identifier_expr_model =
    IdentifierExprModel::new(identifier_model, T::repr());
  ExprHandle::new(Box::new(ExpressionModel::Identifier(identifier_expr_model)))
}

impl<'a, T> ArgTupleHandleMap<'a> for (T,)
  where T: ExprDataType
{
  type HandleTuple = (ExprHandle<'a, T>,);
  type NameTuple = (&'static str,);

  fn make_handle_tuple(names: &Self::NameTuple) -> Self::HandleTuple {
    (make_identifier_expr(names.0),)
  }
  fn make_names_vector(names: &Self::NameTuple) -> Vec<IdentifierModel> {
    vec![IdentifierModel::new(names.0)]
  }
  fn make_types_vector() -> Vec<DataTypeRepr> {
    vec![T::repr()]
  }
  fn visit_argument_handles<V>(handles: Self::HandleTuple, visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>
  {
    visitor.visit_arg(handles.0)
  }
}

impl<'a, T1, T2> ArgTupleHandleMap<'a> for (T1, T2)
  where T1: ExprDataType, T2: ExprDataType
{
  type HandleTuple = (ExprHandle<'a, T1>, ExprHandle<'a, T2>);
  type NameTuple = (&'static str, &'static str);

  fn make_handle_tuple(names: &Self::NameTuple) -> Self::HandleTuple {
    (
      make_identifier_expr(names.0),
      make_identifier_expr(names.1),
    )
  }
  fn make_names_vector(names: &Self::NameTuple) -> Vec<IdentifierModel> {
    vec![IdentifierModel::new(names.0), IdentifierModel::new(names.1)]
  }
  fn make_types_vector() -> Vec<DataTypeRepr> {
    vec![T1::repr(), T2::repr()]
  }
  fn visit_argument_handles<V>(handles: Self::HandleTuple, visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>
  {
    visitor.visit_arg(handles.0);
    visitor.visit_arg(handles.1);
  }
}

impl<'a, T1, T2, T3> ArgTupleHandleMap<'a> for (T1, T2, T3)
  where T1: ExprDataType, T2: ExprDataType, T3: ExprDataType
{
  type HandleTuple = (
    ExprHandle<'a, T1>,
    ExprHandle<'a, T2>,
    ExprHandle<'a, T3>
  );
  type NameTuple = (&'static str, &'static str, &'static str);

  fn make_handle_tuple(names: &Self::NameTuple) -> Self::HandleTuple {
    (
      make_identifier_expr(names.0),
      make_identifier_expr(names.1),
      make_identifier_expr(names.2),
    )
  }
  fn make_names_vector(names: &Self::NameTuple) -> Vec<IdentifierModel> {
    vec![
      IdentifierModel::new(names.0),
      IdentifierModel::new(names.1),
      IdentifierModel::new(names.2),
    ]
  }
  fn make_types_vector() -> Vec<DataTypeRepr> {
    vec![T1::repr(), T2::repr(), T3::repr()]
  }
  fn visit_argument_handles<V>(handles: Self::HandleTuple, visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>
  {
    visitor.visit_arg(handles.0);
    visitor.visit_arg(handles.1);
    visitor.visit_arg(handles.2);
  }
}

impl<'a, T1, T2, T3, T4> ArgTupleHandleMap<'a> for (T1, T2, T3, T4)
  where T1: ExprDataType, T2: ExprDataType, T3: ExprDataType, T4: ExprDataType
{
  type HandleTuple = (
    ExprHandle<'a, T1>,
    ExprHandle<'a, T2>,
    ExprHandle<'a, T3>,
    ExprHandle<'a, T4>
  );
  type NameTuple = (&'static str, &'static str, &'static str, &'static str);

  fn make_handle_tuple(names: &Self::NameTuple) -> Self::HandleTuple {
    (
      make_identifier_expr(names.0),
      make_identifier_expr(names.1),
      make_identifier_expr(names.2),
      make_identifier_expr(names.3),
    )
  }
  fn make_names_vector(names: &Self::NameTuple) -> Vec<IdentifierModel> {
    vec![
      IdentifierModel::new(names.0),
      IdentifierModel::new(names.1),
      IdentifierModel::new(names.2),
      IdentifierModel::new(names.3),
    ]
  }
  fn make_types_vector() -> Vec<DataTypeRepr> {
    vec![T1::repr(), T2::repr(), T3::repr(), T4::repr()]
  }
  fn visit_argument_handles<V>(handles: Self::HandleTuple, visitor: &mut V)
    where V: ArgTupleHandleVisitor<'a>
  {
    visitor.visit_arg(handles.0);
    visitor.visit_arg(handles.1);
    visitor.visit_arg(handles.2);
    visitor.visit_arg(handles.3);
  }
}
