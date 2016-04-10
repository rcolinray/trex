pub struct Halt;

#[macro_export]
macro_rules! simulation {
    {
        components: {
            $( $C:ident : $F:ident ),*
        },

        events: {
            $( $queue:ident : $E:ident ),*
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

        pub struct Events {
            pub halt: $crate::EventQueue<$crate::Halt>,

            $(
                pub $queue: $crate::EventQueue<$E>,
            )*
        }

        impl Events {
            fn new() -> Events {
                Events {
                    halt: $crate::EventQueue::new(),

                    $(
                        $queue: $crate::EventQueue::new(),
                    )*
                }
            }
        }

        pub struct Simulation {
            world: World,
            events: Events,
            received_halt: bool,

            $(
                $system : $S
            ),*
        }

        impl Simulation {
            pub fn new() -> Simulation {
                Simulation {
                    world: {
                        let mut world = World::new();
                        $(
                            world.register_component::<$C>();
                        )+
                        world
                    },
                    events: Events::new(),
                    received_halt: false,
                    $(
                        $system: $S::new()
                    ),*
                }
            }

            pub fn setup<F>(&mut self, setup_fn: F) where F: FnOnce(&mut World, &mut Events) {
                setup_fn(&mut self.world, &mut self.events);
            }

            pub fn update(&mut self, dt: f32) {
                $(
                    self.$system.update(&mut self.world, &mut self.events, dt);
                )*

                $(
                    self.events.$queue.flush();
                )*

                if let Some(_) = self.events.halt.receive().next() {
                    self.received_halt = true;
                }
                self.events.halt.flush();
            }

            pub fn received_halt(&self) -> bool {
                self.received_halt
            }
        }
    }
}
