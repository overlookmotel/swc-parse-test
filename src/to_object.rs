use std::convert::TryInto;
use napi::{CallContext, JsObject, JsString, Result, Env};
use regex::Regex;

// Parse JS string to AST JsObject
#[js_function(1)]
pub fn parse_to_object(ctx: CallContext) -> Result<JsObject> {
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
  let mut span = env.create_object().unwrap();
  span.set_named_property("start", env.create_uint32(start).unwrap()).unwrap();
  span.set_named_property("end", env.create_uint32(end).unwrap()).unwrap();
  span.set_named_property("ctxt", env.create_uint32(0).unwrap()).unwrap();
  span
}
