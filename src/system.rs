pub trait System<T> {
    fn new() -> Self;
    fn update(&mut self, world: &mut T, dt: f32);
}
