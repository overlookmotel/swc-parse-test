#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;
use napi::{CallContext, JsString, JsObject, JsBuffer, Result, Env};
use regex::Regex;

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
  Ok(())
}

// Constants
const TYPE_MODULE: u8 = 0;
const TYPE_VARIABLE_DECLARATION: u8 = 1;
const TYPE_VARIABLE_DECLARATOR: u8 = 2;
const TYPE_IDENTIFIER: u8 = 3;
const TYPE_NUMERIC_LITERAL: u8 = 4;
const VAR_KIND_CONST: u8 = 2;
const NOT_DEFINITE: u8 = 0;
const NOT_OPTIONAL: u8 = 0;

// Parse JS string to AST buffer representation
#[js_function(1)]
fn parse_to_buffer(ctx: CallContext) -> Result<JsBuffer> {
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

// Parse JS string to AST JsObject
#[js_function(1)]
fn parse_to_object(ctx: CallContext) -> Result<JsObject> {
  // Split into lines
  let js_utf8 = ctx.get::<JsString>(0)?.into_utf8()?;
  let js_str: &str = js_utf8.as_str()?;
  let js_len: u32 = js_str.len().try_into().unwrap();
  let lines: Vec<&str> = js_str.trim().split('\n').collect();

  // Parse lines to JS objects
  let regex = Regex::new(r"^const(?P<before_var>\s+)(?P<var>[^ ]+)(?P<before_val>\s*=\s*)(?P<val>\d+);$")
    .unwrap();
  let mut line_start: u32 = 0;

  let env = ctx.env;

  // Module
  let mut module_node = env.create_object()?;
  module_node.set_named_property("type", env.create_string("Module")?)?;
  module_node.set_named_property("span", create_span(env, 0, js_len - 1))?;

  let mut body_nodes = env.create_array()?;
  for (line_num, line) in lines.iter().enumerate() {
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
    let mut declaration_node = env.create_object()?;
    declaration_node.set_named_property("type", env.create_string("VariableDeclaration")?)?;
    declaration_node.set_named_property("span", create_span(env, line_start, line_end))?;
    declaration_node.set_named_property("kind", env.create_string("const")?)?;
    declaration_node.set_named_property("declare", env.get_boolean(false)?)?;

    let mut declaration_nodes = env.create_array()?;

    // VariableDeclarator
    let mut declarator_node = env.create_object()?;
    declarator_node.set_named_property("type", env.create_string("VariableDeclarator")?)?;
    declarator_node.set_named_property("span", create_span(env, var_name_start, line_end - 1))?;

    // Identifier
    let mut identifier_node = env.create_object()?;
    identifier_node.set_named_property("type", env.create_string("Identifier")?)?;
    identifier_node.set_named_property("span", create_span(env, var_name_start, var_name_end))?;
    identifier_node.set_named_property("value", env.create_string(var_name)?)?;
    identifier_node.set_named_property("optional", env.get_boolean(false)?)?;
    identifier_node.set_named_property("typeAnnotation", env.get_null()?)?;
    declarator_node.set_named_property("id", identifier_node)?;

    // NumericLiteral
    let mut init_node = env.create_object()?;
    init_node.set_named_property("type", env.create_string("NumericLiteral")?)?;
    init_node.set_named_property("span", create_span(env, var_name_end + before_val_len, line_end - 1))?;
    init_node.set_named_property("value", env.create_uint32(val)?)?;
    declarator_node.set_named_property("init", init_node)?;

    declarator_node.set_named_property("definite", env.get_boolean(false)?)?;

    declaration_nodes.set_element(0, declarator_node)?;

    declaration_node.set_named_property("declarations", declaration_nodes)?;

    body_nodes.set_element(line_num.try_into().unwrap(), declaration_node)?;

    line_start = line_end + 1;
  }

  module_node.set_named_property("body", body_nodes)?;
  module_node.set_named_property("interpreter", env.get_null()?)?;

  // Return as JsObject
  Ok(module_node)
}

fn create_span(&env: &Env, start: u32, end: u32) -> JsObject {
  let mut span = env.create_object().expect("");
  span.set_named_property("start", env.create_uint32(start).expect("")).expect("");
  span.set_named_property("end", env.create_uint32(end).expect("")).expect("");
  span.set_named_property("ctxt", env.create_uint32(0).expect("")).expect("");
  span
}
