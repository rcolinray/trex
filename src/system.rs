pub trait System<T> {
    fn new() -> Self;

    fn setup(&mut self, _world: &mut T) { }

    fn update(&mut self, world: &mut T, dt: f32);
}
