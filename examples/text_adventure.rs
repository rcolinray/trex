#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate trex;
extern crate ansi_term;

use std::io::{self, Write};
use std::thread::{spawn, sleep};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, SystemTime};

use trex::{Entity, World, System, calc_millis};

use ansi_term::Style;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub entities: Vec<Entity>,
}

impl Room {
    pub fn new(name: &str, description: &str) -> Room {
        Room {
            name: name.to_owned(),
            description: description.to_owned(),
            entities: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Actor {
    pub room: Entity,
}

impl Actor {
    pub fn new(room: Entity) -> Actor {
        Actor {
            room: room,
        }
    }
}

pub struct Input(pub String);

pub struct InputSystem {
    rx: Receiver<String>,
}

impl System<Events> for InputSystem {
    fn new() -> InputSystem {
        let (tx, rx) = channel();

        spawn(move || {
            let stdin = io::stdin();
            loop {
                let mut input = String::new();
                match stdin.read_line(&mut input) {
                    Ok(_) => if let Err(_) = tx.send(input) {
                        break;
                    },
                    Err(_) => break,
                };
            }
        });

        InputSystem {
            rx: rx,
        }
    }

    fn update(&mut self, _world: &mut World, events: &mut Events, _dt: f32) {
        while let Ok(input) = self.rx.try_recv() {
            events.input.emit(Input(input));
        }
    }
}

pub struct Output(pub String);

pub struct OutputSystem;

impl System<Events> for OutputSystem {
    fn new() -> OutputSystem {
        OutputSystem
    }

    fn update(&mut self, _world: &mut World, events: &mut Events, _dt: f32) {
        for &Output(ref output) in events.output.receive() {
            print!("{}", output);
        }

        io::stdout().flush().unwrap();
    }
}

pub struct CommandSystem;

impl System<Events> for CommandSystem {
    fn new() -> CommandSystem {
        CommandSystem
    }

    fn update(&mut self, world: &mut World, events: &mut Events, _dt: f32) {
        for &Input(ref input) in events.input.receive() {
            match input.trim() {
                "look" => {
                    let player = world.lookup("Player").unwrap();
                    let actor = world.get::<Actor>(player).unwrap();
                    let room = world.get::<Room>(actor.room).unwrap();

                    let output = format!("{}\n{}\n",
                        Style::new().bold().underline().paint(room.name.clone()),
                        room.description);
                    events.output.emit(Output(output));
                },

                "quit" => {
                    events.halt.emit(trex::Halt);
                    break;
                },

                _ => {
                    events.output.emit(Output(String::from("Huh?\n")));
                },
            };

            events.output.emit(Output(String::from("> ")));
        }
    }
}

simulation! {
    world: {
        components: {
            Room: ROOM,
            Actor: ACTOR
        },

        events: {
            input: Input,
            output: Output
        }
    },

    systems: {
        input: InputSystem,
        command: CommandSystem,
        output: OutputSystem
    }
}

fn main() {
    let mut simulation = Simulation::new();

    simulation.setup(|world, events| {
        let player = world.create();
        world.tag(player, "Player");
        let entrance = world.create();

        let actor = Actor::new(entrance);
        let mut room = Room::new("Entrance", "You stand at the entrance to a dungeon.");
        room.entities.push(player);

        world.add(player, actor);
        world.add(entrance, room);

        events.output.emit(Output(String::from("> ")));
    });

    let mut last = SystemTime::now();

    loop {
        let now = SystemTime::now();

        if let Ok(dt) = now.duration_since(last) {
            let dt_millis = calc_millis(dt);
            simulation.update(dt_millis);
            last = now;

            if simulation.received_halt() {
                break;
            }
        }

        sleep(Duration::from_millis(1));
    }
}
