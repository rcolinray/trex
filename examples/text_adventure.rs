#[macro_use]
extern crate trex;
extern crate ansi_term;

use std::io::{self, Write};
use std::thread::{spawn, sleep};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, SystemTime};

use trex::{System, EventQueue, EventEmitter, Simulation, World,
           calc_millis, Entity};

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

impl InputSystem {
    pub fn new() -> InputSystem {
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
}

impl System for InputSystem {
    fn update(&mut self, _world: &mut World, _queue: &EventQueue, emitter: &mut EventEmitter, _dt: f32) {
        while let Ok(input) = self.rx.try_recv() {
            emitter.emit(Input(input));
        }
    }
}

pub struct Output(pub String);

pub struct OutputSystem;

impl System for OutputSystem {
    fn update(&mut self, _world: &mut World, queue: &EventQueue, _emitter: &mut EventEmitter, _dt: f32) {
        for &Output(ref output) in queue.receive() {
            print!("{}", output);
        }

        io::stdout().flush().unwrap();
    }
}

pub struct CommandSystem;

impl System for CommandSystem {
    fn update(&mut self, world: &mut World, queue: &EventQueue, emitter: &mut EventEmitter, _dt: f32) {
        for &Input(ref input) in queue.receive() {
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
                    emitter.emit(trex::Halt);
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

components!(Actor, Room);
events!(Input, Output);

fn main() {
    let world = {
        let mut world = World::new();
        world.register::<Actor>();
        world.register::<Room>();

        let player = world.create();
        world.tag(player, "Player");
        let entrance = world.create();

        let actor = Actor::new(entrance);
        let mut room = Room::new("Entrance", "You stand at the entrance to a dungeon.");
        room.entities.push(player);

        world.add(player, actor);
        world.add(entrance, room);
        world
    };

    let queue = {
        let mut queue = EventQueue::new();
        queue.register::<Input>();
        queue.register::<Output>();
        queue
    };

    let mut emitter = {
        let mut emitter = EventEmitter::new();
        emitter.register::<Input>();
        emitter.register::<Output>();
        emitter
    };
    emitter.emit(Output(String::from("> ")));

    let mut simulation = Simulation::new(world, queue, emitter);
    simulation.register(InputSystem::new());
    simulation.register(CommandSystem);
    simulation.register(OutputSystem);

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
