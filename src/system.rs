use super::entity::World;

pub trait System<E> {
    fn new() -> Self;
    fn update(&mut self, world: &mut World, events: &mut E, dt: f32);
}
