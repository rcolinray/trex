#![feature(specialization)]
#[macro_use]
extern crate trex;
extern crate vec_map;
extern crate ansi_term;

use std::io::{self, Write};
use std::thread::{spawn, sleep};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, SystemTime};

use trex::*;

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

components!(Room, Actor);

pub struct Input(pub String);
pub struct Output(pub String);

events!(Input, Output);

pub struct InputSystem {
    rx: Receiver<String>,
}

impl System for InputSystem {
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

    fn update<C, Q, E>(&mut self, _world: &mut World<C>, _queue: &Q, emitter: &mut E, _dt: f32)
        where C: ComponentStorage, Q: EventReceiver<E>, E: EventSender {
        while let Ok(input) = self.rx.try_recv() {
            emitter.emit(Input(input));
        }
    }
}

pub struct OutputSystem;

impl System for OutputSystem {
    fn new() -> OutputSystem {
        OutputSystem
    }

    fn update<C, Q, E>(&mut self, _world: &mut World<C>, queue: &Q, _emitter: &mut E, _dt: f32)
        where C: ComponentStorage, Q: EventReceiver<E>, E: EventSender {
        for &Output(ref output) in queue.receive::<Output>() {
            print!("{}", output);
        }

        io::stdout().flush().unwrap();
    }
}

pub struct CommandSystem;

impl System for CommandSystem {
    fn new() -> CommandSystem {
        CommandSystem
    }

    fn update<C, Q, E>(&mut self, world: &mut World<C>, queue: &Q, emitter: &mut E, _dt: f32)
        where C: ComponentStorage, Q: EventReceiver<E>, E: EventSender {
        for &Input(ref input) in queue.receive::<Input>() {
            match input.trim() {
                "look" => {
                    let player = world.lookup("Player").unwrap();
                    let actor = world.get::<Actor>(player).unwrap();
                    let room = world.get::<Room>(actor.room).unwrap();

                    let output = format!("{}\n{}\n",
                        Style::new().bold().underline().paint(room.name.clone()),
                        room.description);
                    emitter.emit(Output(output));
                },

                "quit" => {
                    emitter.emit(Halt);
                    break;
                },

                _ => {
                    emitter.emit(Output(String::from("Huh?\n")));
                },
            };

            emitter.emit(Output(String::from("> ")));
        }
    }
}

systems!(InputSystem, CommandSystem, OutputSystem);

fn main() {
    let mut simulation = Simulation::<SystemStore, ComponentStore, EventQueue, EventEmitter>::new();

    simulation.setup(|world, emitter| {
        let player = world.create();
        world.tag(player, "Player");
        let entrance = world.create();

        let actor = Actor::new(entrance);
        let mut room = Room::new("Entrance", "You stand at the entrance to a dungeon.");
        room.entities.push(player);

        world.add(player, actor);
        world.add(entrance, room);

        emitter.emit(Output(String::from("> ")));
    });

    let mut last = SystemTime::now();

    loop {
        let now = SystemTime::now();

        if let Ok(dt) = now.duration_since(last) {
            let dt_millis = calc_millis(dt);
            simulation.update(dt_millis);
            last = now;

            if simulation.halt() {
                break;
            }
        }

        sleep(Duration::from_millis(1));
    }
}
