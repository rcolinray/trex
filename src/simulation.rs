use super::family::{Family, FamilyMember};
use super::world::World;
use super::event::{EventQueue, EventEmitter};
use super::system::System;

/// Internal event used to stop the `Simulation`. This event is automatically
/// registered.
pub struct Halt;

impl FamilyMember for Halt {
    fn family() -> Family {
        0
    }
}

/// Responsible for updating and passing events between systems.
pub struct Simulation {
    world: World,
    queue: EventQueue,
    emitter: EventEmitter,
    systems: Vec<Box<System>>,
    halt: bool,
}

impl Simulation {
    /// Create a new `Simulation`.
    pub fn new(world: World, mut queue: EventQueue, mut emitter: EventEmitter) -> Simulation {
        queue.register::<Halt>();
        emitter.register::<Halt>();

        Simulation {
            world: world,
            queue: queue,
            emitter: emitter,
            systems: Vec::new(),
            halt: false,
        }
    }

    /// Register a `System`.
    pub fn register<T: 'static + System>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    /// Returns `true` if the `Halt` event has been emitted, otherwise `false`.
    pub fn halt(&self) -> bool {
        self.halt
    }

    /// Perform a single simulation step.
    pub fn update(&mut self, dt: f32) {
        for system in &mut self.systems {
            system.update(&mut self.world, &self.queue, &mut self.emitter, dt);
            self.queue.merge(&mut self.emitter);
        }

        if let Some(&Halt) = self.queue.receive::<Halt>().next() {
            self.halt = true;
        }

        self.queue.flush();
    }
}
