pub struct Halt;

#[macro_export]
macro_rules! simulation {
    {
        world: {
            components: {
                $( $C:ident : $F:ident ),*
            },

            events: {
                $( $queue:ident : $E:ident ),*
            }
        },

        systems: {
            $( $system:ident : $S:ident ),*
        }
    } => {
        $(
            component! {
                $C : $F
            }
        )*

        pub struct World {
            pub store: $crate::EntityStore,
            pub halt: $crate::EventQueue<$crate::Halt>,

            $(
                pub $queue: $crate::EventQueue<$E>,
            )*
        }

        impl World {
            fn new() -> World {
                World {
                    store: {
                        let mut store = $crate::EntityStore::new();
                        $(
                            store.register_component::<$C>();
                        )*
                        store
                    },
                    halt: $crate::EventQueue::new(),

                    $(
                        $queue: $crate::EventQueue::new(),
                    )*
                }
            }
        }

        pub struct Simulation {
            pub world: World,
            received_halt: bool,

            $(
                $system : $S
            ),*
        }

        impl Simulation {
            pub fn new() -> Simulation {
                Simulation {
                    world: World::new(),
                    received_halt: false,
                    $(
                        $system: $S::new()
                    ),*
                }
            }

            pub fn setup<F>(&mut self, setup_fn: F) where F: FnOnce(&mut World) {
                setup_fn(&mut self.world);
            }

            pub fn update(&mut self, dt: f32) {
                $(
                    self.$system.update(&mut self.world, dt);
                )*

                $(
                    self.world.$queue.flush();
                )*

                if let Some(_) = self.world.halt.receive().next() {
                    self.received_halt = true;
                }
                self.world.halt.flush();
            }

            pub fn received_halt(&self) -> bool {
                self.received_halt
            }
        }
    }
}
