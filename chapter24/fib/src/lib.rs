use napi_derive_ohos::napi;
use napi_ohos::{Env, JsNumber, JsObject, Result, Task};

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

#[napi(ts_return_type="Promise<number>")]
pub fn fib(env: Env, init: u32) -> Result<JsObject> {
    let task = ComputeFib::new(init);
    let async_promise = env.spawn(task).unwrap();
    Ok(async_promise.promise_object())
}
