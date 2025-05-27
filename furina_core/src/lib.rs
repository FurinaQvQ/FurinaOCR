#![feature(decl_macro)]
#![feature(concat_idents)]
#![allow(unused_imports)]

#[cfg(all(feature = "ort", feature = "tract_onnx"))]
compile_error!("feature \"ort\" and \"tract_onnx\" cannot be enabled at the same time");

extern crate lazy_static;
extern crate log;

pub mod capture;
pub mod common;
pub mod export;
pub mod game_info;
pub mod ocr;
pub mod positioning;
pub mod system_control;
pub mod utils;
pub mod window_info;
