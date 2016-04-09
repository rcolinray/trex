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

pub struct ComponentStore<T> {
    map: VecMap<Id>,
    pool: ComponentPool<T>,
}

impl<T> ComponentStore<T> {
    pub fn new() -> ComponentStore<T> {
        ComponentStore {
            map: VecMap::new(),
            pool: ComponentPool::new(),
        }
    }

    pub fn add(&mut self, entity: Entity, data: T) {
        let id = self.pool.add(data);
        self.map.insert(entity, id);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        match self.map.get(entity) {
            Some(&id) => self.pool.get(id),
            None => None,
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        match self.map.get(entity) {
            Some(&id) => self.pool.get_mut(id),
            None => None,
        }
    }

    fn remove(&mut self, entity: Entity) {
        if let Some(&id) = self.map.get(entity) {
            self.pool.remove(id);
        }
    }
}
