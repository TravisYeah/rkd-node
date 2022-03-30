#![deny(clippy::all)]
use napi::bindgen_prelude::Undefined;

#[macro_use]
extern crate napi_derive;

#[napi]
fn create_delta_file(
  source: String,
  target: String,
  delta: String,
  window: i64,
  prime: i64,
) -> Undefined {
  if window < 1 {
    panic!("Window size must be greater than zero.");
  }
  if prime < 1 {
    panic!("Prime must be greater than zero.");
  }
  rk_delta::RabinKarpDelta::create_delta_file(
    source.as_str(),
    target.as_str(),
    delta.as_str(),
    window.try_into().unwrap(),
    prime.try_into().unwrap(),
  );
}

#[napi]
fn create_target_file(source: String, target: String, delta: String) -> Undefined {
  rk_delta::RabinKarpDelta::create_target_file(source.as_str(), target.as_str(), delta.as_str());
}
