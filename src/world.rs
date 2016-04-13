use std::mem::transmute;
use std::collections::HashMap;

use vec_map::VecMap;
use bit_set::BitSet;

use super::id::{Id, IdPool};
use super::component::{AnyComponentStore, InnerComponentStore};
use super::family::FamilyMember;

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
    pub fn with<C: FamilyMember>(mut self) -> Self {
        self.mask.insert(C::family());
        self
    }

    /// Returns `true` if a given entity contains all of the required components, otherwise `false`.
    fn matches(&self, mask: &BitSet) -> bool {
        self.mask.is_subset(mask)
    }
}

/// Used to group components.
pub type Entity = Id;

/// Contains all entities and their components.
pub struct World {
    masks: VecMap<BitSet>,
    stores: VecMap<Box<AnyComponentStore>>,
    pool: IdPool,
    tags: HashMap<String, Entity>,
    tags_by_entity: VecMap<String>,
}

impl World {
    /// Create an empty `World`.
    pub fn new() -> World {
        World {
            masks: VecMap::new(),
            stores: VecMap::new(),
            pool: IdPool::new(),
            tags: HashMap::new(),
            tags_by_entity: VecMap::new(),
        }
    }

    /// Register a new component class.
    pub fn register<C: 'static + FamilyMember>(&mut self) {
        let store = InnerComponentStore::<C>::new();
        self.stores.insert(C::family(), Box::new(store));
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
            self.remove_all_components(entity);
            self.untag(entity);
        }
    }

    fn remove_all_components(&mut self, entity: Entity) {
        let mask = self.masks.get(entity).unwrap();
        for family in mask {
            let store = self.stores.get_mut(family).unwrap();
            store.remove(entity);
        }
    }

    /// Returns a list of all `Entity`s with a given set of components.
    pub fn filter(&self, filter: &ComponentFilter) -> Vec<Entity> {
        self.pool.reserved()
            .filter(|&entity| {
                let mask = self.masks.get(entity).unwrap();
                filter.matches(mask)
            })
            .collect::<Vec<Entity>>()
    }

    /// Attach a component to an `Entity`.
    pub fn add<C: FamilyMember>(&mut self, entity: Entity, component: C) {
        self.set_has_component::<C>(entity, true);
        self.get_store_mut::<C>().add(entity, component);
    }

    /// Remove a component from an `Entity`.
    pub fn remove<C: FamilyMember>(&mut self, entity: Entity) {
        self.set_has_component::<C>(entity, false);
        self.get_store_mut::<C>().remove(entity);
    }

    fn set_has_component<C: FamilyMember>(&mut self, entity: Entity, has_component: bool) {
        let mask = self.masks.get_mut(entity).unwrap();
        let family = C::family();

        if has_component {
            mask.insert(family);
        } else {
            mask.remove(family);
        }
    }

    /// Returns `true` if the `Entity` has the component, otherwise `false`.
    pub fn has<C: FamilyMember>(&self, entity: Entity) -> bool {
        let mask = self.masks.get(entity).unwrap();
        mask.contains(C::family())
    }

    /// Get a component of an `Entity`.
    pub fn get<C: FamilyMember>(&self, entity: Entity) -> Option<&C> {
        let store = self.get_store::<C>();
        store.get(entity)
    }

    /// Get a mutable component of an `Entity`.
    pub fn get_mut<C: FamilyMember>(&mut self, entity: Entity) -> Option<&mut C> {
        let store = self.get_store_mut::<C>();
        store.get_mut(entity)
    }

    fn get_store<C: FamilyMember>(&self) -> &Box<InnerComponentStore<C>> {
        let store = self.stores.get(C::family()).unwrap();
        assert_eq!(store.family(), C::family());
        unsafe { transmute(store) }
    }

    fn get_store_mut<C: FamilyMember>(&mut self) -> &mut Box<InnerComponentStore<C>> {
        let store = self.stores.get_mut(C::family()).unwrap();
        assert_eq!(store.family(), C::family());
        unsafe { transmute(store) }
    }
}
