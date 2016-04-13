use super::system::SystemStore;
use super::world::{ComponentStore, World};
use super::event::{EventQueue, EventEmitter};

pub struct Simulation<S: SystemStore, C: ComponentStore, Q: EventQueue<E>, E: EventEmitter> {
    systems: S,
    world: World<C>,
    queue: Q,
    emitter: E,
}

impl<S: SystemStore, C: ComponentStore, Q: EventQueue<E>, E: EventEmitter> Simulation<S, C, Q, E> {
    pub fn new() -> Simulation<S, C, Q, E> {
        Simulation {
            systems: S::new(),
            world: World::new(),
            queue: Q::new(),
            emitter: E::new(),
        }
    }

    pub fn setup<F>(&mut self, setup: F) where F: FnOnce(&mut World<C>, &mut E) {
        setup(&mut self.world, &mut self.emitter);
    }

    pub fn update(&mut self, dt: f32) {
        self.systems.update(&mut self.world, &mut self.queue, &mut self.emitter, dt);
    }

    pub fn halt(&self) -> bool {
        self.systems.halt()
    }
}