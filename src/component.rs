use vec_map::VecMap;

use super::id::{Id, IdPool};
use super::entity::Entity;

struct ComponentPool<T> {
    data: Vec<T>,
    ids: IdPool,
}

impl<T> ComponentPool<T> {
    fn new() -> ComponentPool<T> {
        ComponentPool {
            data: Vec::new(),
            ids: IdPool::new(),
        }
    }

    fn add(&mut self, data: T) -> Id {
        let id = self.ids.reserve();
        if id < self.data.len() {
            self.data[id] = data;
        } else {
            self.data.push(data);
        }
        id
    }

    fn remove(&mut self, id: Id){
        self.ids.release(id);
    }

    fn get(&self, id: Id) -> Option<&T> {
        if self.ids.is_reserved(id) {
            Some(&self.data[id])
        } else {
            None
        }
    }

    fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        if self.ids.is_reserved(id) {
            Some(&mut self.data[id])
        } else {
            None
        }
    }
}

/// The unique family of a component.
pub type Family = usize;

/// Generates the next unique family. This method should never be called manually. Concrete
/// components should be passed into the `simulation!` macro, which will setup the families.
pub unsafe fn next_family() -> Family {
    static mut NEXT_FAMILY: Family = 0;

    let next = NEXT_FAMILY;
    NEXT_FAMILY += 1;
    next
}

/// Trait implemented by all components in order to distinguish stores of different components.
/// this trait should never by implemented manually. Concrete components should be passed into the
/// `simulation!` macro, which will implement this trait for each component.
pub trait Component: 'static {
    /// Returns the unique `Family` for the `Component`.
    fn family() -> Family;
}

/// Used to implement the component trait for a given type. This macro should never be called
/// manually. Concrete components should be passed into the `simulation!` macro, which will call
/// this macro.
#[macro_export]
macro_rules! component {
    { $C:ident : $F:ident } => {
        lazy_static! {
            pub static ref $F: $crate::Family = unsafe { $crate::next_family() };
        }

        impl $crate::Component for $C {
            fn family() -> $crate::Family {
                *$F
            }
        }
    }
}

pub trait AnyComponentStore {
    fn family(&self) -> Family;
    fn remove(&mut self, entity: Entity);
}

pub struct ComponentStore<C: Component> {
    map: VecMap<Id>,
    pool: ComponentPool<C>,
}

impl<C: Component> ComponentStore<C> {
    pub fn new() -> ComponentStore<C> {
        ComponentStore {
            map: VecMap::new(),
            pool: ComponentPool::new(),
        }
    }

    pub fn add(&mut self, entity: Entity, data: C) {
        let id = self.pool.add(data);
        self.map.insert(entity, id);
    }

    pub fn get(&self, entity: Entity) -> Option<&C> {
        match self.map.get(entity) {
            Some(&id) => self.pool.get(id),
            None => None,
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        match self.map.get(entity) {
            Some(&id) => self.pool.get_mut(id),
            None => None,
        }
    }
}

impl<C: Component> AnyComponentStore for ComponentStore<C> {
    fn family(&self) -> Family {
        C::family()
    }

    fn remove(&mut self, entity: Entity) {
        if let Some(&id) = self.map.get(entity) {
            self.pool.remove(id);
        }
    }
}

