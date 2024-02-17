use napi::{CallContext, Env, JsNumber, JsObject, Result, Task};
use napi_derive::{js_function, module_exports};

struct ComputeFib {
  n: u32,
}


fn fibonacci_native(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
  }
}

impl ComputeFib {
  pub fn new(n: u32) -> ComputeFib {
    ComputeFib { n }
  }
}

impl Task for ComputeFib {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(fibonacci_native(self.n))
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

#[js_function(0)]
pub fn native_fib(ctx: CallContext) -> Result<JsObject> {
  let task = ComputeFib::new(50);
  let async_promise = ctx.env.spawn(task)?;
  Ok(async_promise.promise_object())
}

#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("nativeUVFib", native_fib)?;
  Ok(())
}
