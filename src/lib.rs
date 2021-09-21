#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]

//use std::rc::Rc;

pub mod applicative;
pub mod bifunctor;
pub mod bimapable;
pub mod boxed;
pub mod functor;
pub mod mapable;
pub mod ref_into_iterator;
pub mod ref_mapable;
//pub mod pairmapable;

pub use applicative::{Applicative, ApplicativeOnce};
pub use bifunctor::{BiCoherent, BiTypeMap, BiFunctor, BiFunctorOnce};
pub use boxed::*;
pub use crate::functor::*;
pub use mapable::*;
pub use ref_into_iterator::*;
