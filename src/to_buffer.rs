use std::convert::TryInto;
use napi::{CallContext, JsString, JsBuffer, Result};
use regex::Regex;

// Constants
const TYPE_MODULE: u8 = 0;
const TYPE_VARIABLE_DECLARATION: u8 = 1;
const TYPE_VARIABLE_DECLARATOR: u8 = 2;
const TYPE_IDENTIFIER: u8 = 3;
const TYPE_NUMERIC_LITERAL: u8 = 4;
const VAR_KIND_CONST: u8 = 2;
const NOT_DEFINITE: u8 = 0;
const NOT_OPTIONAL: u8 = 0;

// Parse JS string to AST serialized as a buffer with custom encoding
#[js_function(1)]
pub fn parse_to_buffer(ctx: CallContext) -> Result<JsBuffer> {
  // Split into lines
  let js_utf8 = ctx.get::<JsString>(0)?.into_utf8()?;
  let js_str: &str = js_utf8.as_str()?;
  let js_len: u32 = js_str.len().try_into().unwrap();
  let lines: Vec<&str> = js_str.trim().split('\n').collect();

  // Parse lines to bytes
  let regex = Regex::new(r"^const(?P<before_var>\s+)(?P<var>[^ ]+)(?P<before_val>\s*=\s*)(?P<val>\d+);$")
    .unwrap();

  let mut bytes: Vec<u8> = Vec::new();
  let mut line_start: u32 = 0;

  // Module
  push_u8(&mut bytes, TYPE_MODULE);
  push_span(&mut bytes, 0, js_len - 1);
  let num_lines: u32 = lines.len().try_into().unwrap();
  push_u32(&mut bytes, num_lines);

  for line in &lines {
    // Parse line
    let caps = regex.captures(line).unwrap();

    let line_len: u32 = line.len().try_into().unwrap();
    let line_end = line_start + line_len;
    let var_name: &str = &caps["var"];
    let before_var_len: u32 = caps["before_var"].len().try_into().unwrap();
    let var_name_start: u32 = line_start + before_var_len + 5;
    let var_name_len: u16 = var_name.len().try_into().unwrap();
    let var_name_len_32: u32 = var_name_len.try_into().unwrap();
    let var_name_end = var_name_start + var_name_len_32;
    let val: u32 = caps["val"].parse::<u32>().unwrap();
    let before_val_len: u32 = caps["before_val"].len().try_into().unwrap();

    // VariableDeclaration
    push_u8(&mut bytes, TYPE_VARIABLE_DECLARATION);
    push_span(&mut bytes, line_start, line_end);
    push_u8(&mut bytes, VAR_KIND_CONST);
    push_u32(&mut bytes, 1); // Number of declarators

    // VariableDeclarator
    push_u8(&mut bytes, TYPE_VARIABLE_DECLARATOR);
    push_span(&mut bytes, var_name_start, line_end - 1);
    push_u8(&mut bytes, NOT_DEFINITE);

    // Identifier
    push_u8(&mut bytes, TYPE_IDENTIFIER);
    push_span(&mut bytes, var_name_start, var_name_end);
    push_u8(&mut bytes, NOT_OPTIONAL);
    push_u16(&mut bytes, var_name_len);
    for c in var_name.bytes() {
      push_u8(&mut bytes, c);
    }

    // NumericLiteral
    push_u8(&mut bytes, TYPE_NUMERIC_LITERAL);
    push_span(&mut bytes, var_name_end + before_val_len, line_end - 1);
    push_u32(&mut bytes, val);

    line_start = line_end + 1;
  }

  // Return bytes as JsBuffer
  Ok(ctx.env.create_buffer_with_data(bytes)?.into_raw())
}

fn push_u8(bytes: &mut Vec<u8>, val: u8) {
  bytes.push(val);
}

fn push_u16(bytes: &mut Vec<u8>, val: u16) {
  let byte1 = val % 256;
  let byte2 = (val - byte1) / 256;
  bytes.push(byte1.try_into().unwrap());
  bytes.push(byte2.try_into().unwrap());
}

fn push_u32(bytes: &mut Vec<u8>, val: u32) {
  let byte1 = val % 256;
  let rem1 = (val - byte1) / 256;
  let byte2 = rem1 % 256;
  let rem2 = (rem1 - byte2) / 256;
  let byte3 = rem2 % 256;
  let byte4 = (rem2 - byte3) / 256;
  bytes.push(byte1.try_into().unwrap());
  bytes.push(byte2.try_into().unwrap());
  bytes.push(byte3.try_into().unwrap());
  bytes.push(byte4.try_into().unwrap());
}

fn push_span(bytes: &mut Vec<u8>, start: u32, end: u32) {
  push_u32(bytes, start);
  push_u32(bytes, end);
  push_u8(bytes, 0); // ctxt
}
