#![feature(coroutines, coroutine_trait)]
#![feature(fn_traits)]
#![feature(stmt_expr_attributes)]

//! # Genshin Impact Specific Implementations for FurinaOCR
//! 
//! This module contains Genshin Impact specific implementations for the FurinaOCR system.

pub mod application;
pub mod artifact;
pub mod character;
pub mod export;
pub mod scanner;
pub mod scanner_controller;
