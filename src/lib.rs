#![allow(bare_trait_objects, non_snake_case, unused_mut, unused_imports, dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate url;

pub mod apis;
pub mod models;
