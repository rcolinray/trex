#[macro_export]
macro_rules! family {
    ( @$family:expr, ) => {};

    ( @$family:expr, $head:ident, $( $tail:ident, )* ) => {
        impl $crate::FamilyMember for $head {
            fn family() -> $crate::Family {
                $family
            }
        }

        family!(@$family + 1, $( $tail, )*);
    };

    ( $( $T:ident ),+ ) => {
        family!(@0, $( $T, )+ );
    };
}

#[macro_export]
macro_rules! events {
    ([ $( $T:ident ),+ ]) => {
        family!( $( $T ),+ );

        pub enum EventQueueWrapper {
            $(
                $T($crate::InnerEventQueue<$T>)
            ),+
        }

        impl EventQueueWrapper {
            pub fn flush(&mut self) {
                match self {
                    $(
                        &mut EventQueueWrapper::$T(ref mut queue) => queue.flush()
                    ),+
                }
            }

            pub fn merge(&mut self, emitter_wrapper: &mut EventEmitterWrapper) {
                match (self, emitter_wrapper) {
                    $(
                        (&mut EventQueueWrapper::$T(ref mut queue), &mut EventEmitterWrapper::$T(ref mut emitter)) => queue.merge(emitter),
                    )+
                    _ => panic!("merge called with different queue and emitter event types"),
                }
            }
        }

        impl<T> $crate::Wrapper<$crate::InnerEventQueue<T>> for EventQueueWrapper {
            default fn get_inner(&self) -> &$crate::InnerEventQueue<T> {
                unimplemented!();
            }

            default fn get_inner_mut(&mut self) -> &mut $crate::InnerEventQueue<T> {
                unimplemented!();
            }
        }

        $(
            impl $crate::Wrapper<$crate::InnerEventQueue<$T>> for EventQueueWrapper {
                fn get_inner(&self) -> &$crate::InnerEventQueue<$T> {
                    match self {
                        &EventQueueWrapper::$T(ref inner) => inner,
                        _ => panic!("get_inner called with unexpected event type"),
                    }
                }

                fn get_inner_mut(&mut self) -> &mut $crate::InnerEventQueue<$T> {
                    match self {
                        &mut EventQueueWrapper::$T(ref mut inner) => inner,
                        _ => panic!("get_inner_mut called with unexpected event type"),
                    }
                }
            }
        )+

        impl $crate::FamilyStore for EventQueueWrapper {
            fn family(&self) -> $crate::Family {
                match self {
                    $(
                        &EventQueueWrapper::$T(_) => $T::family()
                    ),+
                }
            }
        }

        pub enum EventEmitterWrapper {
            $(
                $T($crate::InnerEventEmitter<$T>)
            ),+
        }

        impl<T> $crate::Wrapper<$crate::InnerEventEmitter<T>> for EventEmitterWrapper {
            default fn get_inner(&self) -> &$crate::InnerEventEmitter<T> {
                unimplemented!();
            }

            default fn get_inner_mut(&mut self) -> &mut $crate::InnerEventEmitter<T> {
                unimplemented!();
            }
        }

        $(
            impl $crate::Wrapper<$crate::InnerEventEmitter<$T>> for EventEmitterWrapper {
                fn get_inner(&self) -> &$crate::InnerEventEmitter<$T> {
                    match self {
                        &EventEmitterWrapper::$T(ref inner) => inner,
                        _ => panic!("get_inner called with unexpected event type"),
                    }
                }

                fn get_inner_mut(&mut self) -> &mut $crate::InnerEventEmitter<$T> {
                    match self {
                        &mut EventEmitterWrapper::$T(ref mut inner) => inner,
                        _ => panic!("get_inner_mut called with unexpected event type"),
                    }
                }
            }
        )+

        impl $crate::FamilyStore for EventEmitterWrapper {
            fn family(&self) -> $crate::Family {
                match self {
                    $(
                        &EventEmitterWrapper::$T(_) => $T::family()
                    ),+
                }
            }
        }

        pub struct EventQueue {
            wrappers: vec_map::VecMap<EventQueueWrapper>,
        }

        impl $crate::EventReceiver<EventEmitter> for EventQueue {
            fn new() -> EventQueue {
                EventQueue {
                    wrappers: {
                        let mut map = vec_map::VecMap::new();
                        $(
                            map.insert($T::family(), EventQueueWrapper::$T($crate::InnerEventQueue::new()));
                        )+
                        map
                    },
                }
            }

            fn receive<T: $crate::FamilyMember>(&self) -> $crate::Iter<T> {
                let family = T::family();
                match self.wrappers.get(family) {
                    Some(wrapper) => {
                        let queue = <$crate::Wrapper<$crate::InnerEventQueue<T>>>::get_inner(wrapper);
                        queue.receive()
                    },
                    None => panic!("receive called with unexpected event type"),
                }
            }

            fn flush(&mut self) {
                for (_, wrapper) in self.wrappers.iter_mut() {
                    wrapper.flush();
                }
            }

            fn merge(&mut self, emitter: &mut EventEmitter) {
                for (family, emitter_wrapper) in emitter.wrappers.iter_mut() {
                    if let Some(wrapper) = self.wrappers.get_mut(family) {
                        wrapper.merge(emitter_wrapper);
                    }
                }
            }
        }

        pub struct EventEmitter {
            wrappers: vec_map::VecMap<EventEmitterWrapper>,
        }

        impl $crate::EventSender for EventEmitter {
            fn new() -> EventEmitter {
                EventEmitter {
                    wrappers: {
                        let mut map = vec_map::VecMap::new();
                        $(
                            map.insert($T::family(), EventEmitterWrapper::$T($crate::InnerEventEmitter::new()));
                        )+
                        map
                    },
                }
            }

            fn emit<T: $crate::FamilyMember>(&mut self, event: T) {
                let family = T::family();
                match self.wrappers.get_mut(family) {
                    Some(wrapper) => {
                        let queue = <$crate::Wrapper<$crate::InnerEventEmitter<T>>>::get_inner_mut(wrapper);
                        queue.emit(event);
                    },
                    None => panic!("emit called with unexpected event type"),
                };
            }
        }
    };

    ( $( $T:ident ),+ ) => {
        pub struct Halt;

        events!([ Halt, $( $T ),+ ]);
    };
}

#[macro_export]
macro_rules! components {
    ( $( $T:ident ),+ ) => {
        family!( $( $T ),+ );

        pub enum ComponentStoreWrapper {
            $(
                $T($crate::InnerComponentStore<$T>)
            ),+
        }

        impl ComponentStoreWrapper {
            pub fn remove(&mut self, entity: $crate::Entity) {
                match self {
                    $(
                        &mut ComponentStoreWrapper::$T(ref mut store) => store.remove(entity)
                    ),+
                };
            }
        }

        impl<T> $crate::Wrapper<$crate::InnerComponentStore<T>> for ComponentStoreWrapper {
            default fn get_inner(&self) -> &$crate::InnerComponentStore<T> {
                unimplemented!();
            }

            default fn get_inner_mut(&mut self) -> &mut $crate::InnerComponentStore<T> {
                unimplemented!();
            }
        }

        $(
            impl $crate::Wrapper<$crate::InnerComponentStore<$T>> for ComponentStoreWrapper {
                fn get_inner(&self) -> &$crate::InnerComponentStore<$T> {
                    match self {
                        &ComponentStoreWrapper::$T(ref inner) => inner,
                        _ => panic!("get_inner called with unexpected component type"),
                    }
                }

                fn get_inner_mut(&mut self) -> &mut $crate::InnerComponentStore<$T> {
                    match self {
                        &mut ComponentStoreWrapper::$T(ref mut inner) => inner,
                        _ => panic!("get_inner_mut called with unexpected component type"),
                    }
                }
            }
        )+

        impl $crate::FamilyStore for ComponentStoreWrapper {
            fn family(&self) -> $crate::Family {
                match self {
                    $(
                        &ComponentStoreWrapper::$T(_) => $T::family()
                    ),+
                }
            }
        }

        pub struct  ComponentStore {
            wrappers: vec_map::VecMap<ComponentStoreWrapper>,
        }

        impl $crate::ComponentStorage for ComponentStore {
            fn new() -> ComponentStore {
                ComponentStore {
                    wrappers: {
                        let mut map = vec_map::VecMap::new();
                        $(
                            map.insert($T::family(), ComponentStoreWrapper::$T($crate::InnerComponentStore::new()));
                        )+
                        map
                    },
                }
            }

            fn add<T: $crate::FamilyMember>(&mut self, entity: $crate::Entity, component: T) {
                let family = T::family();
                match self.wrappers.get_mut(family) {
                    Some(wrapper) => {
                        let store = <$crate::Wrapper<$crate::InnerComponentStore<T>>>::get_inner_mut(wrapper);
                        store.add(entity, component);
                    },
                    None => panic!("add called with unexpected component type"),
                };
            }

            fn get<T: $crate::FamilyMember>(&self, entity: $crate::Entity) -> Option<&T> {
                let family = T::family();
                match self.wrappers.get(family) {
                    Some(wrapper) => {
                        let store = <$crate::Wrapper<$crate::InnerComponentStore<T>>>::get_inner(wrapper);
                        store.get(entity)
                    },
                    None => panic!("get called with unexpected component type"),
                }
            }

            fn get_mut<T: $crate::FamilyMember>(&mut self, entity: $crate::Entity) -> Option<&mut T> {
                let family = T::family();
                match self.wrappers.get_mut(family) {
                    Some(wrapper) => {
                        let store = <$crate::Wrapper<$crate::InnerComponentStore<T>>>::get_inner_mut(wrapper);
                        store.get_mut(entity)
                    },
                    None => panic!("get_mut called with unexpected component type"),
                }
            }

            fn remove<T: $crate::FamilyMember>(&mut self, entity: $crate::Entity) {
                let family = T::family();
                match self.wrappers.get_mut(family) {
                    Some(wrapper) => {
                        let store = <$crate::Wrapper<$crate::InnerComponentStore<T>>>::get_inner_mut(wrapper);
                        store.remove(entity);
                    },
                    None => panic!("remove called with unexpected component type"),
                };
            }

            fn remove_all(&mut self, entity: $crate::Entity) {
                for (_, wrapper) in self.wrappers.iter_mut() {
                    wrapper.remove(entity);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! systems {
    ( $( $T:ident ),+ ) => {
        pub enum SystemWrapper {
            $(
                $T($T)
            ),+
        }

        impl SystemWrapper {
            fn update<C, R, S>(&mut self, world: &mut World<C>, queue: &R, emitter: &mut S, dt: f32)
                where C: $crate::ComponentStorage, R: $crate::EventReceiver<S>, S: $crate::EventSender {
                match self {
                    $(
                        &mut SystemWrapper::$T(ref mut system) => system.update(world, queue, emitter, dt)
                    ),+
                }
            }
        }

        pub struct SystemStore {
            systems: Vec<SystemWrapper>,
            halt: bool,
        }

        impl $crate::SystemStorage for SystemStore {
            fn new() -> SystemStore {
                SystemStore {
                    systems: {
                        let mut list = Vec::new();
                        $(
                            list.push(SystemWrapper::$T($T::new()));
                        )+
                        list
                    },
                    halt: false,
                }
            }

            fn halt(&self) -> bool {
                self.halt
            }

            fn update<C, R, S>(&mut self, world: &mut World<C>, queue: &mut R, emitter: &mut S, dt: f32)
                where C: $crate::ComponentStorage, R: $crate::EventReceiver<S>, S: $crate::EventSender {
                for wrapper in self.systems.iter_mut() {
                    wrapper.update(world, queue, emitter, dt);
                    queue.merge(emitter);
                }

                if let Some(&Halt) = queue.receive::<Halt>().next() {
                    self.halt = true;
                }

                queue.flush();
            }
        }
    }
}