use std::collections::HashMap;

use vec_map::VecMap;

use super::id;

pub type Entity = id::Id;

pub struct EntityStore {
    pool: id::IdPool,
    tags: HashMap<String, Entity>,
    tags_by_entity: VecMap<String>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            pool: id::IdPool::new(),
            tags: HashMap::new(),
            tags_by_entity: VecMap::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        self.pool.reserve()
    }

    pub fn tag(&mut self, entity: Entity, tag: &'static str) {
        self.tags.insert(tag.to_owned(), entity);
        self.tags_by_entity.insert(entity, tag.to_owned());
    }

    pub fn get(&self, tag: &'static str) -> Option<Entity> {
        self.tags.get(&tag.to_owned()).cloned()
    }

    pub fn destroy(&mut self, entity: Entity) {
        self.pool.release(entity);
        if let Some(tag) = self.tags_by_entity.remove(entity) {
            self.tags.remove(&tag);
        }
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
