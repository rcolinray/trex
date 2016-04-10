#[macro_use]
extern crate lazy_static;
extern crate vec_map;
extern crate bit_set;

#[macro_use]
mod component;
mod entity;
mod event;
mod id;
#[macro_use]
mod simulation;
mod system;
mod time;

pub use component::{Component, Family, next_family, ComponentStore};
pub use entity::{Entity, World};
pub use event::EventQueue;
pub use simulation::Halt;
pub use system::System;
pub use time::calc_millis;
