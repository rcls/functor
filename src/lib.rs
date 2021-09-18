#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
// #![feature(associated_type_bounds)]

//use std::rc::Rc;

pub mod functor;
pub mod mapable;
pub mod derived_mapable;

pub use crate::functor::*;
pub use mapable::*;
pub use derived_mapable::*;
