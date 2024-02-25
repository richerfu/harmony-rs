use crc32c::crc32c_append;
use crc32fast::Hasher;
use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Either, JsBuffer, Result};

#[napi(js_name = "crc32c")]
pub fn crc32c(input: Either<String, JsBuffer>, initial_state: Option<u32>) -> Result<u32> {
  Ok(match input {
    Either::A(s) => crc32c_append(initial_state.unwrap_or(0), s.as_bytes()),
    Either::B(b) => crc32c_append(initial_state.unwrap_or(0), &b.into_value()?),
  })
}

#[napi]
pub fn crc32(input: Either<String, JsBuffer>, initial_state: Option<u32>) -> Result<u32> {
  let mut hasher = Hasher::new_with_initial(initial_state.unwrap_or(0));
  match input {
    Either::A(s) => {
      hasher.update(s.as_bytes());
    }
    Either::B(b) => {
      let b = b.into_value()?;
      hasher.update(&b);
    }
  };
  Ok(hasher.finalize())
}
