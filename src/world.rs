use std::collections::HashMap;

use vec_map::VecMap;
use bit_set::BitSet;

use super::id::{Id, IdPool};
use super::family::FamilyMember;

/// Used to group components.
pub type Entity = Id;

/// Used to filter the list of entities based on the components that are attached to them.
pub struct ComponentFilter {
    mask: BitSet,
}

impl ComponentFilter {
    /// Create an empty `ComponentFilter`.
    pub fn new() -> ComponentFilter {
        ComponentFilter {
            mask: BitSet::new(),
        }
    }

    /// Extend the filter to include the given component type.
    pub fn with<T: FamilyMember>(mut self) -> Self {
        self.mask.insert(T::family());
        self
    }

    /// Returns `true` if a given entity contains all of the required components, otherwise `false`.
    fn matches(&self, mask: &BitSet) -> bool {
        self.mask.is_subset(mask)
    }
}

pub trait ComponentStorage {
    fn new() -> Self;
    fn add<T: FamilyMember>(&mut self, entity: Entity, component: T);
    fn get<T: FamilyMember>(&self, entity: Entity) -> Option<&T>;
    fn get_mut<T: FamilyMember>(&mut self, entity: Entity) -> Option<&mut T>;
    fn remove<T: FamilyMember>(&mut self, entity: Entity);
    fn remove_all(&mut self, entity: Entity);
}

/// Contains all entities and their components.
pub struct World<T: ComponentStorage> {
    masks: VecMap<BitSet>,
    store: T,
    pool: IdPool,
    tags: HashMap<String, Entity>,
    tags_by_entity: VecMap<String>,
}

impl<T: ComponentStorage> World<T> {
    /// Create an empty `World`.
    pub fn new() -> World<T> {
        World {
            masks: VecMap::new(),
            store: T::new(),
            pool: IdPool::new(),
            tags: HashMap::new(),
            tags_by_entity: VecMap::new(),
        }
    }

    /// Returns `true` if the entity has been created and is not destroyed, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = trex::World::new();
    /// let entity = world.create();
    /// assert!(world.exists(entity));
    /// ```
    pub fn exists(&self, entity: Entity) -> bool {
        self.pool.is_reserved(entity)
    }

    /// Create a new `Entity`.
    pub fn create(&mut self) -> Entity {
        let entity = self.pool.reserve();
        self.accomodate_entity(entity);
        entity
    }

    fn accomodate_entity(&mut self, entity: Entity) {
        if self.masks.contains_key(entity) {
            self.masks.get_mut(entity).unwrap().clear();
        } else {
            self.masks.insert(entity, BitSet::new());
        }
    }

    /// Assign a tag to the `Entity` so that it can be retrieved later.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = trex::World::new();
    /// let entity = world.create();
    /// world.tag(entity, "Example");
    /// assert_eq!(world.lookup("Example"), Some(entity));
    /// ```
    pub fn tag(&mut self, entity: Entity, tag: &str) {
        if self.exists(entity) {
            self.tags.insert(tag.to_owned(), entity);
            self.tags_by_entity.insert(entity, tag.to_owned());
        }
    }

    /// Remove the existing tag, if any, from an `Entity`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = trex::World::new();
    /// let entity = world.create();
    /// world.tag(entity, "Example");
    /// world.untag(entity);
    /// assert_eq!(world.lookup("Example"), None);
    /// ```
    pub fn untag(&mut self, entity: Entity) {
        if let Some(tag) = self.tags_by_entity.remove(entity) {
            self.tags.remove(&tag);
        }
    }

    /// Retreive an `Entity` using a tag.
    pub fn lookup(&self, tag: &str) -> Option<Entity> {
        let owned = tag.to_owned();
        self.tags.get(&owned).cloned()
    }

    /// Destroy an existing `Entity`. Also removes the tag and any attached components.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut world = trex::World::new();
    /// let entity = world.create();
    /// world.tag(entity, "Example");
    /// world.destroy(entity);
    /// assert_eq!(world.lookup("Example"), None);
    /// ```
    pub fn destroy(&mut self, entity: Entity) {
        if self.exists(entity) {
            self.pool.release(entity);
            self.store.remove_all(entity);
            self.untag(entity);
        }
    }

    /// Returns a list of all `Entity`s with a given set of `Component`s.
    pub fn filter_entities(&self, filter: &ComponentFilter) -> Vec<Entity> {
        self.pool.reserved()
            .filter(|&entity| {
                let mask = self.masks.get(entity).unwrap();
                filter.matches(mask)
            })
            .collect::<Vec<Entity>>()
    }

    /// Attach a `Component` to an `Entity`.
    pub fn add<U: FamilyMember>(&mut self, entity: Entity, component: U) {
        self.set_has_component::<U>(entity, true);
        self.store.add(entity, component);
    }

    /// Remove a `Component` from an `Entity`.
    pub fn remove<U: FamilyMember>(&mut self, entity: Entity) {
        self.set_has_component::<U>(entity, false);
        self.store.remove::<U>(entity);
    }

    fn set_has_component<U: FamilyMember>(&mut self, entity: Entity, has_component: bool) {
        let mask = self.masks.get_mut(entity).unwrap();
        let family = U::family();

        if has_component {
            mask.insert(family);
        } else {
            mask.remove(family);
        }
    }

    /// Returns `true` if the `Entity` has the `Component`, otherwise `false`.
    pub fn has<U: FamilyMember>(&self, entity: Entity) -> bool {
        let mask = self.masks.get(entity).unwrap();
        mask.contains(U::family())
    }

    /// Get a `Component` of an `Entity`.
    pub fn get<U: FamilyMember>(&self, entity: Entity) -> Option<&U> {
        self.store.get::<U>(entity)
    }

    /// Get a mutable `Component` of an `Entity`.
    pub fn get_mut<U: FamilyMember>(&mut self, entity: Entity) -> Option<&mut U> {
        self.store.get_mut::<U>(entity)
    }
}
