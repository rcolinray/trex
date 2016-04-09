pub type Id = usize;

pub struct IdPool {
    reserved: Vec<bool>,
    released: Vec<Id>,
}

impl IdPool {
    pub fn new() -> IdPool {
        IdPool {
            reserved: Vec::new(),
            released: Vec::new(),
        }
    }

    pub fn exists(&self, id: Id) -> bool {
        id < self.reserved.len()
    }

    pub fn reserved(&self) -> Iter {
        Iter::new(self)
    }

    pub fn reserve(&mut self) -> Id {
        match self.released.pop() {
            Some(id) => id,
            None => {
                let id = self.reserved.len();
                self.reserved.push(true);
                id
            }
        }
    }

    pub fn is_reserved(&self, id: Id) -> bool {
        assert!(self.exists(id));
        self.reserved[id]
    }

    pub fn release(&mut self, id: Id) {
        if self.is_reserved(id) {
            self.reserved[id] = false;
            self.released.push(id);
        }
    }
}

pub struct Iter<'a> {
    pool: &'a IdPool,
    id: Id,
}

impl<'a> Iter<'a> {
    fn new(pool: &IdPool) -> Iter {
        Iter {
            pool: pool,
            id: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Id> {
        loop {
            let id = self.id;
            self.id += 1;

            if !self.pool.exists(id) {
                return None;
            } else if self.pool.is_reserved(id) {
                return Some(id);
            }
        }
    }
}
