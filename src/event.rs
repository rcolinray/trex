use std::slice::Iter as SliceIter;
use std::mem::transmute;

use vec_map::VecMap;

use super::family::{Family, FamilyMember, FamilyStore};

trait AnyEventQueue: FamilyStore {
    fn flush(&mut self);
    fn merge(&mut self, any_emitter: &mut Box<AnyEventEmitter>);
}

trait AnyEventEmitter: FamilyStore { }

/// Allows for emitting events and iterating over them in the order that they were emitted.
/// Events are immutable and queues are not flushed until the end of the frame, so multiple
/// clients can receive events from the same queue.
///
/// # Examples
///
/// ```
/// let mut queue = trex::EventQueue::<&str>::new();
/// queue.emit("Hello!");
/// assert_eq!(queue.receive().next(), Some(&"Hello!"));
/// ```
struct InnerEventQueue<T> {
    events: Vec<T>,
}

impl<T> InnerEventQueue<T> {
    /// Create a new, empty `InnerEventQueue`.
    fn new() -> InnerEventQueue<T> {
        InnerEventQueue {
            events: Vec::new(),
        }
    }

    /// Iterate over all events in the queue.
    fn receive(&self) -> Iter<T> {
        Iter::new(self.events.iter())
    }
}

impl<T: FamilyMember> FamilyStore for InnerEventQueue<T> {
    fn family(&self) -> Family {
        T::family()
    }
}

impl<T: FamilyMember> AnyEventQueue for InnerEventQueue<T> {
    /// Clear all events from the queue.
    fn flush(&mut self) {
        self.events.clear();
    }

    fn merge(&mut self, any_emitter: &mut Box<AnyEventEmitter>) {
        assert_eq!(self.family(), any_emitter.family());
        let emitter: &mut Box<InnerEventEmitter<T>> = unsafe { transmute(any_emitter) };
        self.events.append(&mut emitter.events);
    }
}

pub struct Iter<'a, T: 'a> {
    iter: SliceIter<'a, T>,
}

impl<'a, T: 'a> Iter<'a, T> {
    fn new(iter: SliceIter<'a, T>) -> Iter<'a, T> {
        Iter {
            iter: iter,
        }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
}

struct InnerEventEmitter<T> {
    events: Vec<T>,
}

impl<T> InnerEventEmitter<T> {
    /// Create a new, empty `InnerEventEmitter`.
    fn new() -> InnerEventEmitter<T> {
        InnerEventEmitter {
            events: Vec::new(),
        }
    }

    /// Emit a new event to the queue.
    fn emit(&mut self, event: T) {
        self.events.push(event);
    }
}

impl<T: FamilyMember> FamilyStore for InnerEventEmitter<T> {
    fn family(&self) -> Family {
        T::family()
    }
}

impl<T: FamilyMember> AnyEventEmitter for InnerEventEmitter<T> {

}

pub struct EventQueue {
    queues: VecMap<Box<AnyEventQueue>>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queues: VecMap::new(),
        }
    }

    pub fn register<T: 'static + FamilyMember>(&mut self) {
        self.queues.insert(T::family(), Box::new(InnerEventQueue::<T>::new()));
    }

    pub fn receive<T: FamilyMember>(&self) -> Iter<T> {
        let any_queue = self.queues.get(T::family()).unwrap();
        let queue: &Box<InnerEventQueue<T>> = unsafe { transmute(any_queue) };
        queue.receive()
    }

    pub fn flush(&mut self) {
        for (_, any_queue) in self.queues.iter_mut() {
            any_queue.flush();
        }
    }

    pub fn merge(&mut self, emitter: &mut EventEmitter) {
        for (family, any_emitter) in emitter.emitters.iter_mut() {
            let any_queue = self.queues.get_mut(family).unwrap();
            any_queue.merge(any_emitter);
        }
    }
}

pub struct EventEmitter {
    emitters: VecMap<Box<AnyEventEmitter>>,
}

impl EventEmitter {
    pub fn new() -> EventEmitter {
        EventEmitter {
            emitters: VecMap::new(),
        }
    }

    pub fn register<T: 'static + FamilyMember>(&mut self) {
        self.emitters.insert(T::family(), Box::new(InnerEventEmitter::<T>::new()));
    }

    pub fn emit<T: FamilyMember>(&mut self, event: T) {
        let any_emitter = self.emitters.get_mut(T::family()).unwrap();
        let emitter: &mut Box<InnerEventEmitter<T>> = unsafe { transmute(any_emitter) };
        emitter.emit(event);
    }
}

