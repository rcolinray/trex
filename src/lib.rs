//! An entity component system inspired by _entityx_.
//!
//! # Examples
//!
//! A simple 2D physics simulation.
//!
//! ```
//! #[macro_use]
//! extern crate trex;
//!
//! use trex::*;
//!
//! // The components used in the simulation.
//! pub struct Position { pub x: f32, pub y: f32 }
//! pub struct Velocity { pub dx: f32, pub dy: f32 }
//! pub struct Acceleration { pub ddx: f32, pub ddy: f32 }
//!
//! components!(Position, Velocity, Acceleration);
//!
//! pub struct PhysicsSystem {
//!     filter: ComponentFilter, // Used to select entities with the components of interest to this
//!                              // system.
//! }
//!
//! impl PhysicsSystem {
//!     pub fn new() -> PhysicsSystem {
//!         PhysicsSystem {
//!             filter: ComponentFilter::new()
//!                 .with::<Position>()
//!                 .with::<Velocity>()
//!                 .with::<Acceleration>(),
//!         }
//!     }
//! }
//!
//! impl System for PhysicsSystem {
//!     fn update(&mut self, world: &mut World, queue: &EventQueue, emitter: &mut EventEmitter, dt: f32) {
//!         let dt_secs = dt / 1000.0;
//!         for entity in world.filter(&self.filter) {
//!             assert!(world.has::<Position>(entity));
//!             assert!(world.has::<Velocity>(entity));
//!             assert!(world.has::<Acceleration>(entity));
//!             let (dx, dy) = {
//!                 let &Acceleration { ddx, ddy } = world.get::<Acceleration>(entity).unwrap();
//!                 let mut vel = world.get_mut::<Velocity>(entity).unwrap();
//!                 vel.dx += ddx * dt_secs;
//!                 vel.dy += ddy * dt_secs;
//!                 (vel.dx, vel.dy)
//!             };
//!             let mut pos = world.get_mut::<Position>(entity).unwrap();
//!             pos.x += dx * dt_secs;
//!             pos.y += dy * dt_secs;
//!         }
//!         emitter.emit(trex::Halt);
//!     }
//! }
//!
//! struct TestSystem;
//!
//! impl System for TestSystem {
//!     fn update(&mut self, world: &mut World, _queue: &EventQueue, _emitter: &mut EventEmitter, _dt: f32) {
//!         let entity = world.lookup("Test").unwrap();
//!         let pos = world.get::<Position>(entity).unwrap();
//!         assert_eq!(pos.x, 9.0);
//!         assert_eq!(pos.y, 12.0);
//!     }
//! }
//!
//! fn main() {
//!     let world = {
//!         let mut world = World::new();
//!         world.register::<Position>();
//!         world.register::<Velocity>();
//!         world.register::<Acceleration>();
//!
//!         // Create an entity that accelerates in the x and y directions.
//!         let entity = world.create();
//!         world.tag(entity, "Test");
//!         world.add(entity, Position { x: 1.0, y: 2.0 });
//!         world.add(entity, Velocity { dx: 3.0, dy: 4.0 });
//!         world.add(entity, Acceleration { ddx: 5.0, ddy: 6.0 });
//!         world
//!     };
//!
//!     let mut queue = EventQueue::new();
//!     let mut emitter = EventEmitter::new();
//!
//!     let mut simulation = Simulation::new(world, queue, emitter);
//!     simulation.register(PhysicsSystem::new());
//!     simulation.register(TestSystem);
//!
//!     // Run a single iteration of the simulation.
//!     simulation.update(1000.0);
//!     assert!(simulation.halt());
//! }
//! ```

extern crate vec_map;
extern crate bit_set;

mod component;
mod event;
mod family;
mod id;
mod simulation;
mod system;
mod time;
mod world;

#[macro_use]
mod macros;

pub use family::{Family, FamilyMember};
pub use event::{EventQueue, EventEmitter};
pub use simulation::{Halt, Simulation};
pub use system::System;
pub use time::calc_millis;
pub use world::{ComponentFilter, Entity, World};
