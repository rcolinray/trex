pub trait Wrapper<T> {
    fn get_inner(&self) -> &T;
    fn get_inner_mut(&mut self) -> &mut T;
}