use crate::shader_repr::{
  data_type::ShaderDataTypeUseRepr,
  module::ShaderModuleRepr,
  native_type::{ ShaderNativeTypeRepr, ShaderNativeScalarTypeRepr },
  structure::{ ShaderStructureRepr, ShaderStructureFieldRepr },
};

/**
 * Generates a shader module string from a `ShaderModuleRepr`
 */
pub(crate) fn generate_shader_module(shader_module: &ShaderModuleRepr) {
  let mut buffer = Vec::<String>::new();
}

struct GeneratorState {
  types_buffer: GeneratorBuffer,
  indent: usize,
}
struct GeneratorBuffer {
  indent: usize,
  buffer: Vec<String>,
}
impl GeneratorState {
  /** Internal generator. */
  fn gen_shader_module(&mut self, shader_module: &ShaderModuleRepr) {
    self.gen_uniforms(&shader_module.uniforms);
  }

  /** Generate the uniforms structure. */
  fn gen_uniforms(&mut self, uniforms: &ShaderStructureRepr) {
    self.gen_structure(uniforms);
  }

  /** Generate a structure definition. */
  fn gen_structure(&mut self, structure: &ShaderStructureRepr) {
    self.types_buffer.push_line_multi(&[ "struct ", &structure.name, " {" ]);
    self.inc_indent();
    for field in &structure.fields {
      self.gen_structure_field(field);
    }
    self.dec_indent();
    self.types_buffer.push_line("}");
  }

  /** Generate a structure field definition. */
  fn gen_structure_field(&mut self,
    field: &ShaderStructureFieldRepr,
  ) {
    self.types_buffer.push_tabspace();
    self.types_buffer.push_raw_multi(&[&field.name, ": "]);
    self.gen_data_type_use(&field.data_type);
  }

  /** Generate a data type reference. */
  fn gen_data_type_use(&mut self, data_type: &ShaderDataTypeUseRepr) {
    match data_type {
      ShaderDataTypeUseRepr::Native(ref native) => {
        match native {
          ShaderNativeTypeRepr::Scalar(ref scalar) => {
            self.gen_native_scalar_type(scalar);
          }
          ShaderNativeTypeRepr::Vec2(ref scalar) => {
            self.types_buffer.push_raw("vec2<");
            self.gen_native_scalar_type(scalar);
            self.types_buffer.push_raw(">");
          }
          ShaderNativeTypeRepr::Vec3(ref scalar) => {
            self.types_buffer.push_raw("vec3<");
            self.gen_native_scalar_type(scalar);
            self.types_buffer.push_raw(">");
          }
          ShaderNativeTypeRepr::Vec4(ref scalar) => {
            self.types_buffer.push_raw("vec4<");
            self.gen_native_scalar_type(scalar);
            self.types_buffer.push_raw(">");
          }
        }
      },
      ShaderDataTypeUseRepr::Structure(ref name) => {
        self.types_buffer.push_raw(&name);
      },
    }
  }

  /** Generate a reference to a native scalar type. */
  fn gen_native_scalar_type(&mut self,
    native_scalar_type: &ShaderNativeScalarTypeRepr
  ) {
    match native_scalar_type {
      ShaderNativeScalarTypeRepr::I32 => {
        self.types_buffer.push_raw("i32");
      }
      ShaderNativeScalarTypeRepr::U32 => {
        self.types_buffer.push_raw("u32");
      }
      ShaderNativeScalarTypeRepr::F32 => {
        self.types_buffer.push_raw("f32");
      }
    }
  }

  /** Increment the indent. */
  fn inc_indent(&mut self) {
    self.indent += 1;
    self.types_buffer.indent += 1;
  }

  /** Increment the indent. */
  fn dec_indent(&mut self) {
    self.indent -= 1;
    self.types_buffer.indent -= 1;
  }
}
impl GeneratorBuffer {
  const INDENT_WIDTH: usize = 4;

  /** Push a tabbed line to the buffer. */
  fn push_line_multi(&mut self, s: &[&str]) {
    self.push_tabspace();
    for part in s {
      self.push_raw(part);
    }
    self.push_newline();
  }

  /** Push a tabbed line to the buffer. */
  fn push_line(&mut self, s: &str) {
    self.push_tabspace();
    self.push_raw(s);
    self.push_newline();
  }

  /** Push a newline to the buffer. */
  fn push_newline(&mut self) {
    self.buffer.push("\n".to_string());
  }

  /**
   * Push the whitespace at the start of a line as per the current
   * tab level.
   */
  fn push_tabspace(&mut self) {
    self.buffer.push("  ".repeat(self.indent * Self::INDENT_WIDTH));
  }

  /** Push a string to the buffer. */
  fn push_raw(&mut self, s: &str) {
    self.buffer.push(s.to_string());
  }

  /** Push a string to the buffer. */
  fn push_raw_multi(&mut self, s: &[&str]) {
    for part in s {
      self.push_raw(part);
    }
  }
}

