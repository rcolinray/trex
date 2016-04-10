use std::mem::transmute;
use std::collections::HashMap;

use vec_map::VecMap;
use bit_set::BitSet;

use super::id::{Id, IdPool};
use super::component::{Component, AnyComponentStore, ComponentStore};

pub struct ComponentFilter {
    mask: BitSet,
}

impl ComponentFilter {
    pub fn new() -> ComponentFilter {
        ComponentFilter {
            mask: BitSet::new(),
        }
    }

    pub fn has<C: Component>(mut self) -> Self {
        self.mask.insert(C::family());
        self
    }

    fn matches(&self, mask: &BitSet) -> bool {
        self.mask.is_subset(mask)
    }
}

pub type Entity = Id;

pub struct EntityStore {
    masks: VecMap<BitSet>,
    stores: VecMap<Box<AnyComponentStore>>,
    pool: IdPool,
    tags: HashMap<String, Entity>,
    tags_by_entity: VecMap<String>,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            masks: VecMap::new(),
            stores: VecMap::new(),
            pool: IdPool::new(),
            tags: HashMap::new(),
            tags_by_entity: VecMap::new(),
        }
    }

    pub fn register_component<C: Component>(&mut self) {
        let store = ComponentStore::<C>::new();
        self.stores.insert(C::family(), Box::new(store));
    }

    pub fn exists(&self, entity: Entity) -> bool {
        self.pool.is_reserved(entity)
    }

    pub fn create(&mut self) -> Entity {
        let entity = self.pool.reserve();
        self.accomodate_entity(entity);
        entity
    }

    fn accomodate_entity(&mut self, entity: Entity) {
        if self.masks.contains_key(entity) {
            self.masks.get_mut(entity).unwrap().clear();
            self.clear_tag(entity);
        } else {
            self.masks.insert(entity, BitSet::new());
        }
    }

    pub fn tag(&mut self, entity: Entity, tag: &str) {
        if self.exists(entity) {
            self.tags.insert(tag.to_owned(), entity);
            self.tags_by_entity.insert(entity, tag.to_owned());
        }
    }

    fn clear_tag(&mut self, entity: Entity) {
        if let Some(tag) = self.tags_by_entity.remove(entity) {
            self.tags.remove(&tag);
        }
    }

    pub fn lookup(&self, tag: &str) -> Option<Entity> {
        let owned = tag.to_owned();
        self.tags.get(&owned).cloned()
    }

    pub fn destroy(&mut self, entity: Entity) {
        if self.exists(entity) {
            self.pool.release(entity);
            self.remove_all_components(entity);
        }
    }

    fn remove_all_components(&mut self, entity: Entity) {
        let mask = self.masks.get(entity).unwrap();
        for family in mask {
            let store = self.stores.get_mut(family).unwrap();
            store.remove(entity);
        }
    }

    pub fn filter_entities(&self, filter: &ComponentFilter) -> Vec<Entity> {
        self.pool.reserved()
            .filter(|&entity| {
                let mask = self.masks.get(entity).unwrap();
                filter.matches(mask)
            })
            .collect::<Vec<Entity>>()
    }

    pub fn add<C: Component>(&mut self, entity: Entity, component: C) {
        self.set_has_component::<C>(entity, true);
        self.get_store_mut::<C>().add(entity, component);
    }

    pub fn remove<C: Component>(&mut self, entity: Entity) {
        self.set_has_component::<C>(entity, false);
        self.get_store_mut::<C>().remove(entity);
    }

    fn set_has_component<C: Component>(&mut self, entity: Entity, has_component: bool) {
        let mask = self.masks.get_mut(entity).unwrap();
        let family = C::family();

        if has_component {
            mask.insert(family);
        } else {
            mask.remove(family);
        }
    }

    pub fn has<C: Component>(&self, entity: Entity) -> bool {
        let mask = self.masks.get(entity).unwrap();
        mask.contains(C::family())
    }

    pub fn get<C: Component>(&self, entity: Entity) -> Option<&C> {
        let store = self.get_store::<C>();
        store.get(entity)
    }

    pub fn get_mut<C: Component>(&mut self, entity: Entity) -> Option<&mut C> {
        let store = self.get_store_mut::<C>();
        store.get_mut(entity)
    }

    fn get_store<C: Component>(&self) -> &Box<ComponentStore<C>> {
        let store = self.stores.get(C::family()).unwrap();
        assert_eq!(store.family(), C::family());
        unsafe { transmute(store) }
    }

    fn get_store_mut<C: Component>(&mut self) -> &mut Box<ComponentStore<C>> {
        let store = self.stores.get_mut(C::family()).unwrap();
        assert_eq!(store.family(), C::family());
        unsafe { transmute(store) }
    }
}
