use vec_map::VecMap;

use super::id::{Id, IdPool};
use super::world::Entity;
use super::family::{Family, FamilyMember, FamilyStore};

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

pub trait AnyComponentStore: FamilyStore {
    fn remove(&mut self, entity: Entity);
}

pub struct InnerComponentStore<C: FamilyMember> {
    map: VecMap<Id>,
    pool: ComponentPool<C>,
}

impl<C: FamilyMember> InnerComponentStore<C> {
    pub fn new() -> InnerComponentStore<C> {
        InnerComponentStore {
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

impl<C: FamilyMember> FamilyStore for InnerComponentStore<C> {
    fn family(&self) -> Family {
        C::family()
    }
}

impl<C: FamilyMember> AnyComponentStore for InnerComponentStore<C> {
    fn remove(&mut self, entity: Entity) {
        if let Some(&id) = self.map.get(entity) {
            self.pool.remove(id);
        }
    }
}

