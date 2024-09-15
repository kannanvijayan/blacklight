use crate::{
  api::data_type::{ DataTypeRepr, HostShareableDataType, StructFieldRepr },
  model::IdentifierModel,
};

/**
 * Structs are used to create custom data types.
 */
pub trait StructMappedDataType: 'static + Sized {
  const NAME: &'static str;
  fn visit_fields<FV>(fv: &mut FV)
    where FV: StructFieldVisitor;
}

/**
 * A visitor interface for struct fields.
 */
pub trait StructFieldVisitor {
  fn visit_field<DT>(&mut self, name: &str)
    where DT: HostShareableDataType;
}

/**
 * The actual rust "struct" type that maps to the various data type traits
 * is not directly the implementor of StructMappedDataType.
 * 
 * Instead, it is `Struct<T>`.  For this type to be instantiable, T must be
 * Copy and implement StructMappedDataType.
 */
pub struct Struct<T: Copy + StructMappedDataType> {
  // The struct data.
  data: T,
}
impl<T: Copy + StructMappedDataType> Struct<T> {
  /** Construct a repr for this struct type. */
  pub(crate) fn make_repr() -> DataTypeRepr {
    struct Visitor {
      fields: Vec<StructFieldRepr>,
    }
    impl StructFieldVisitor for Visitor {
      fn visit_field<DT>(&mut self, name: &str)
        where DT: HostShareableDataType
      {
        let ident_model = IdentifierModel::new(name);
        self.fields.push(StructFieldRepr::new(ident_model, DT::repr()));
      }
    }

    let mut visitor = Visitor { fields: Vec::new() };
    T::visit_fields(&mut visitor);

    DataTypeRepr::new_struct(IdentifierModel::new(T::NAME), visitor.fields)
  }
}
impl<T: Copy + StructMappedDataType> Clone for Struct<T> {
  fn clone(&self) -> Self {
    Struct { data: self.data }
  }
}
impl<T: Copy + StructMappedDataType> Copy for Struct<T> {
}
