use super::world::World;
use super::event::{EventQueue, EventEmitter};

/// Trait that must be implemented by all systems in the `Simulation`.
pub trait System {
    /// This method is called each frame, giving the `System` access to the `World`, `EventQueue`,
    /// and `EventEmitter`. `dt` is the time in milliseconds since the last update.
    fn update(&mut self, world: &mut World, queue: &EventQueue, emitter: &mut EventEmitter, dt: f32);
}
