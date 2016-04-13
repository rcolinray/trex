use std::slice;

use super::family::FamilyMember;

pub trait EventReceiver<T: EventSender> {
    fn new() -> Self;
    fn receive<U: FamilyMember>(&self) -> Iter<U>;
    fn flush(&mut self);
    fn merge(&mut self, sender: &mut T);
}

pub trait EventSender {
    fn new() -> Self;
    fn emit<T: FamilyMember>(&mut self, event: T);
}

pub struct InnerEventQueue<T> {
    events: Vec<T>,
}

impl<T> InnerEventQueue<T> {
    pub fn new() -> InnerEventQueue<T> {
        InnerEventQueue {
            events: Vec::new(),
        }
    }

    pub fn receive(&self) -> Iter<T> {
        Iter::new(self.events.iter())
    }

    pub fn flush(&mut self) {
        self.events.clear();
    }

    pub fn merge(&mut self, emitter: &mut InnerEventEmitter<T>) {
        self.events.append(&mut emitter.events);
    }
}

pub struct InnerEventEmitter<T> {
    events: Vec<T>,
}

impl<T> InnerEventEmitter<T> {
    pub fn new() -> InnerEventEmitter<T> {
        InnerEventEmitter {
            events: Vec::new(),
        }
    }

    pub fn emit(&mut self, event: T) {
        self.events.push(event);
    }
}

pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

impl<'a, T: 'a> Iter<'a, T> {
    fn new(iter: slice::Iter<'a, T>) -> Iter<'a, T> {
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