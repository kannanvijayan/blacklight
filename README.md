
# Blacklight

A helper library for defining wgsl shaders, typically for use with the
rust `wgsl` library.


## Code organization

  * `util/` - utility code that does not depend on internally defined structures.
    - `shared` - Rhe `Shared` type helps defer choice between `Rc` and `Arc`.
  * `model/` - crate-public static-lifetime models of declared structures
  * `api/` - public definitions with nice lifetime annotations
    - `project.rs` - top-level project encapsulation
    - `shader.rs` - top-level shader encapsulation
    - `entry_point.rs` - top-level entrypoint encapsulation
    - `data_type/` - type-mappings from rust types to classes of wgsl types
    - `builder/` - procedural builder API for constructing shaders
    - `handle/` - lifetime-annotated model handles for use within builders.
  * `printer` - the code to generate pretty wgsl text from a model.
    
The general expectation is that things in `model/` share their underlying
contents and are cloneable, and have static lifetime.

Likewise, the top-level `Project` and `Shader` and `EntryPoint` types are
static-lifetime values that share their underlying data and can be cloned.

The types in `api/builder/` however, use lifetimes to constrain the mixing
of declarations and enforce scope-compliance.

