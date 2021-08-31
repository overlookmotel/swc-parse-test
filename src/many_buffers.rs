use std::convert::TryInto;
use napi::{CallContext, JsObject, JsString, Result};


// Parse JS string to multiple small buffers.
// This doesn't actually do any parsing. Each line of code is converted to 4 small Buffers containing
// useless data. 4 Buffers per line to represent the 4 AST nodes per line.
// This is just to test, if it were possible to pass raw Rust memory to JS by getting pointers
// to types and turning each into a Buffer, whether it would be performant passing them to JS.
// (Answer is no!)
#[js_function(1)]
pub fn parse_to_many_buffers(ctx: CallContext) -> Result<JsObject> {
  // Split into lines
  let js_utf8 = ctx.get::<JsString>(0)?.into_utf8()?;
  let js_str: &str = js_utf8.as_str()?;
  // let js_len: u32 = js_str.len().try_into().unwrap();
  let lines: Vec<&str> = js_str.trim().split('\n').collect();

  let mut buffers = ctx.env.create_array()?;
  for (line_num, _line) in lines.iter().enumerate() {
    let line_num_u32: u32 = line_num.try_into().unwrap();

    let bytes1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let buf1 = ctx.env.create_buffer_with_data(bytes1)?.into_raw();
    buffers.set_element(line_num_u32 * 4, buf1)?;

    let bytes2 = vec![17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let buf2 = ctx.env.create_buffer_with_data(bytes2)?.into_raw();
    buffers.set_element(line_num_u32 * 4 + 1, buf2)?;

    let bytes3 = vec![33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 27, 48];
    let buf3 = ctx.env.create_buffer_with_data(bytes3)?.into_raw();
    buffers.set_element(line_num_u32 * 4 + 2, buf3)?;

    let bytes4 = vec![49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64];
    let buf4 = ctx.env.create_buffer_with_data(bytes4)?.into_raw();
    buffers.set_element(line_num_u32 * 4 + 3, buf4)?;
  }
  Ok(buffers)
}
