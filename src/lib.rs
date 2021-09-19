#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
// #![feature(associated_type_bounds)]

//use std::rc::Rc;

pub mod applicative;
pub mod bifunctor;
pub mod bimapable;
pub mod boxed;
pub mod functor;
pub mod mapable;

pub use bifunctor::{BiCoherent, BiTypeMap, BiFunctor, BiFunctorOnce};
pub use boxed::*;
pub use crate::functor::*;
pub use mapable::*;
