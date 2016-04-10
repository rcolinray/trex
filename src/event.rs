use std::slice;

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
pub struct EventQueue<T> {
    events: Vec<T>,
}

impl<T> EventQueue<T> {
    /// Create a new, empty `EventQueue`.
    pub fn new() -> EventQueue<T> {
        EventQueue {
            events: Vec::new(),
        }
    }

    /// Emit a new event to the queue.
    pub fn emit(&mut self, event: T) {
        self.events.push(event);
    }

    /// Iterate over all events in the queue.
    pub fn receive(&self) -> Iter<T> {
        Iter::new(self.events.iter())
    }

    /// Clear all events from the queue.
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

