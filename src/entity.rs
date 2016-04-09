use std::collections::HashMap;

use super::id;

pub type Entity = id::Id;

pub struct EntityStore {
    pool: id::IdPool,
    tags: HashMap<&'static str, Entity>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            pool: id::IdPool::new(),
            tags: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        self.pool.reserve()
    }

    pub fn tag(&mut self, entity: Entity, tag: &'static str) {
        self.tags.insert(tag, entity);
    }

    pub fn get(&self, tag: &'static str) -> Option<Entity> {
        self.tags.get(&tag).cloned()
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.pool.release(entity);
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.pool.exists(entity) && self.pool.is_reserved(entity)
    }

    pub fn entities(&self) -> Iter {
        Iter::new(self.pool.reserved())
    }
}

pub struct Iter<'a> {
    iter: id::Iter<'a>,
}

impl<'a> Iter<'a> {
    fn new(iter: id::Iter<'a>) -> Iter {
        Iter {
            iter: iter,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Entity> {
        self.iter.next()
    }
}