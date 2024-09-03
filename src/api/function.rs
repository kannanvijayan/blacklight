use std::marker::PhantomData;
use crate::api::data_type::{ShArgTupleDataType, ProcResultType};

/**
 * A project represents a collection of type definitions, constants,
 * and functions that can be used to specify and execute shader modules.
 */
pub struct Function<ReturnT, ArgsT>
  where ReturnT: ProcResultType,
        ArgsT: ShArgTupleDataType
{
  _phantom: PhantomData<(ReturnT, ArgsT)>
}
