use std::slice;

pub struct EventQueue<T> {
    events: Vec<T>,
}

impl<T> EventQueue<T> {
    pub fn new() -> EventQueue<T> {
        EventQueue {
            events: Vec::new(),
        }
    }

    pub fn emit(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn receive(&self) -> Iter<T> {
        Iter::new(self.events.iter())
    }

    pub fn flush(&mut self) {
        self.events.clear();
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

