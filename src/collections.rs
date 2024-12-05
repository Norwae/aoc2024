use std::array;

#[derive(Debug, Clone)]
pub struct IndexMap<T, const N: usize> {
    storage: [Option<T>; N],
    mappings: usize,
}

impl<T, const N: usize> IndexMap<T, N> {
    pub fn new() -> Self {
        Self { storage: [const { None }; N], mappings: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.mappings == 0
    }

    pub fn remove(&mut self, index: usize) {
        if self.storage[index].is_some() {
            self.storage[index] = None;
            self.mappings -= 1;
        }
    }

    pub fn values_iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(usize, &mut T)> {
        self.storage.iter_mut().enumerate().filter_map(|(n, stored)| {
            match stored {
                None => None,
                Some(v) => Some((n, v))
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item=(usize, &T)> {
        self.storage.iter().enumerate().filter_map(|(n, stored)| {
            match stored {
                None => None,
                Some(v) => Some((n, v))
            }
        })
    }
}

impl<T: Default, const N: usize> IndexMap<T, N> {
    pub fn get_or_insert_default(&mut self, n: usize) -> &mut T {
        let mut stored = &mut self.storage[n];
        if stored.is_none() {
            self.mappings += 1;
        }
        stored.get_or_insert_default()
    }
}
