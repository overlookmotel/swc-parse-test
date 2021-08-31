#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{JsObject, Result};

use self::{
  to_buffer::{parse_to_buffer},
  to_object::{parse_to_object},
  many_buffers::{parse_to_many_buffers}
};

mod to_buffer;
mod to_object;
mod many_buffers;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("parseToBuffer", parse_to_buffer)?;
  exports.create_named_method("parseToObject", parse_to_object)?;
  exports.create_named_method("parseToManyBuffers", parse_to_many_buffers)?;
  Ok(())
}
