use crate::shader_repr::{
  body::ShaderBodyRepr,
  data_type::{ ShaderDataTypeUseRepr, ShaderReturnDataTypeUseRepr },
};

pub(crate) struct ShaderFunctionRepr {
  name: String,
  return_type: Option<ShaderReturnDataTypeUseRepr>,
  arguments: Vec<ShaderFunctionArgumentRepr>,
  body: ShaderBodyRepr,
}

pub(crate) struct ShaderFunctionArgumentRepr {
  name: String,
  data_type: ShaderDataTypeUseRepr,
}
