use std::marker::PhantomData;
use crate::api::data_type::{ShArgTupleDataType, ShProcResultType};

/**
 * A project represents a collection of type definitions, constants,
 * and functions that can be used to specify and execute shader modules.
 */
pub struct Function<ReturnT, ArgsT>
  where ReturnT: ShProcResultType,
        ArgsT: ShArgTupleDataType
{
  _phantom: PhantomData<(ReturnT, ArgsT)>
}
