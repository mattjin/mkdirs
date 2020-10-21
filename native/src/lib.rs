use neon::prelude::*;
use std::fs;
use neon::register_module;

fn mkdirs(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let str = cx.argument::<JsString>(0)?.value();
  fs::create_dir_all(&str).unwrap();
  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("mkdirs", mkdirs)?;
  Ok(())
});