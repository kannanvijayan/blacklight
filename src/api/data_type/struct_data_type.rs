use std::marker::PhantomData;
use crate::{
  api::data_type::{ DataTypeRepr, HostShareableDataType, StructDataTypeRepr, StructFieldRepr },
  model::IdentifierModel,
};

/**
 * Structs are used to create custom data types.
 */
pub trait StructMappedDataType: 'static + Copy + Sized {
  const NAME: &'static str;
  fn visit_fields<FV>(fv: &mut FV)
    where FV: StructFieldVisitor<Self>;
}

/**
 * A visitor interface for struct fields.
 */
pub trait StructFieldVisitor<T: StructMappedDataType> {
  fn visit_field<DT, GET, SET>(&mut self, name: &str, get: GET, set: SET)
    where DT: HostShareableDataType,
          GET: Fn(&T) -> DT,
          SET: Fn(&mut T, DT);
}

/**
 * The actual rust "struct" type that maps to the various data type traits
 * is not directly the implementor of StructMappedDataType.
 * 
 * Instead, it is `Struct<T>`.  For this type to be instantiable, T must be
 * Copy and implement StructMappedDataType.
 */
pub struct Struct<T: StructMappedDataType> {
  // The struct data.
  data: T,
}
impl<T: StructMappedDataType> Struct<T> {
  /** Construct a repr for this struct type. */
  pub(crate) fn make_struct_repr() -> StructDataTypeRepr {
    let mut visitor = MakeReprVisitor::<T> {
      fields: Vec::new(),
      _phantom: PhantomData,
    };
    T::visit_fields(&mut visitor);

    StructDataTypeRepr::new(IdentifierModel::new(T::NAME), visitor.fields)
  }

  /** Construct a repr for this struct type. */
  pub(crate) fn make_repr() -> DataTypeRepr {
    DataTypeRepr::new_struct(Self::make_struct_repr())
  }

  /** Get the underlying data. */
  pub fn data(&self) -> &T {
    &self.data
  }
}
impl<T: StructMappedDataType> Clone for Struct<T> {
  fn clone(&self) -> Self {
    Struct { data: self.data }
  }
}
impl<T: StructMappedDataType> Copy for Struct<T> {
}
impl<T: StructMappedDataType> From<T> for Struct<T> {
  fn from(data: T) -> Self {
    Struct { data: data }
  }
}

struct MakeReprVisitor<T> {
  fields: Vec<StructFieldRepr>,
  _phantom: PhantomData<T>,
}
impl<T> StructFieldVisitor<T> for MakeReprVisitor<T>
  where T: StructMappedDataType
{
  fn visit_field<DT, GET, SET>(&mut self, name: &str, _get: GET, _set: SET)
    where DT: HostShareableDataType,
          GET: Fn(&T) -> DT,
          SET: Fn(&mut T, DT)
  {
    let ident_model = IdentifierModel::new(name);
    self.fields.push(StructFieldRepr::new(ident_model, DT::repr()));
  }
}
