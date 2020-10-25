use neon::prelude::*;
use std::fs;
use neon::register_module;

struct DirTask {
  argument: String,
}

impl Task for DirTask {
  type Output = usize;
  type Error = ();
  type JsEvent = JsUndefined;

  fn perform(&self) -> Result<usize, ()> {
      fs::create_dir_all(&self.argument).unwrap();
      Ok(1)
  }

  fn complete(self, mut cx: TaskContext, _result: Result<usize, ()>) -> JsResult<JsUndefined> {
      Ok(cx.undefined())
  }
}

fn mkdirs_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let str = cx.argument::<JsString>(0)?.value();
  fs::create_dir_all(&str).unwrap();
  Ok(cx.undefined())
}

fn mkdirs_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let str = cx.argument::<JsString>(0)?.value();
  let cb = cx.argument::<JsFunction>(1)?;

  let task = DirTask { argument: str };
  task.schedule(cb);

  Ok(cx.undefined())
}

register_module!(mut cx, {
  cx.export_function("mkdirsSync", mkdirs_sync)?;
  cx.export_function("mkdirsAsync", mkdirs_async)?;
  Ok(())
});