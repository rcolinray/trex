use super::system::SystemStorage;
use super::world::{ComponentStorage, World};
use super::event::{EventReceiver, EventSender};

pub struct Simulation<S: SystemStorage, C: ComponentStorage, Q: EventReceiver<E>, E: EventSender> {
    systems: S,
    world: World<C>,
    queue: Q,
    emitter: E,
}

impl<S: SystemStorage, C: ComponentStorage, Q: EventReceiver<E>, E: EventSender> Simulation<S, C, Q, E> {
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