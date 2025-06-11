pub struct BinaryVec<T> {
    vec: Vec<T>,
}

impl<T: Ord> BinaryVec<T> {
    /// Creates a new empty `BinaryVec`.
    pub fn new() -> Self {
        BinaryVec { vec: Vec::new() }
    }

    /// Creates a new `BinaryVec` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        BinaryVec {
            vec: Vec::with_capacity(capacity),
        }
    }

    /// Returns the vector internally used by `BinaryVec`.
    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }

    /// Inserts a value into the `BinaryVec`, maintaining sorted order, and returns the index where
    /// the value was inserted.
    pub fn insert(&mut self, value: T) -> usize {
        match self.vec.binary_search(&value) {
            Ok(index) => index,
            Err(index) => {
                self.vec.insert(index, value);

                index
            }
        }
    }

    /// Returns the item at the specified index, or `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    /// Returns the index of the specified value, or `None` if the value is not found.
    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.vec.binary_search(value).ok()
    }

    /// Removes the item at the specified index, returning it if it exists, or `None` if the index
    /// is out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.vec.len() {
            Some(self.vec.remove(index))
        } else {
            None
        }
    }

    /// Removes the specified item from the `BinaryVec`, returning it if it exists, or `None` if the
    /// item is not found.
    pub fn remove_item(&mut self, value: &T) -> Option<T> {
        match self.vec.binary_search(value) {
            Ok(index) => Some(self.vec.remove(index)),
            Err(_) => None,
        }
    }
}

impl<T: Ord> Default for BinaryVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut binary_vec = BinaryVec::new();
        assert_eq!(binary_vec.insert(5), 0);
        assert_eq!(binary_vec.insert(3), 0);
        assert_eq!(binary_vec.insert(7), 2);
        assert_eq!(binary_vec.get(0), Some(&3));
        assert_eq!(binary_vec.get(1), Some(&5));
        assert_eq!(binary_vec.get(2), Some(&7));
    }

    #[test]
    fn test_remove() {
        let mut binary_vec = BinaryVec::new();
        binary_vec.insert(5);
        binary_vec.insert(3);
        binary_vec.insert(7);

        assert_eq!(binary_vec.remove(1), Some(5));
        assert_eq!(binary_vec.get(0), Some(&3));
        assert_eq!(binary_vec.get(1), Some(&7));
        assert_eq!(binary_vec.remove(10), None); // Out of bounds
    }

    #[test]
    fn test_remove_item() {
        let mut binary_vec = BinaryVec::new();
        binary_vec.insert(5);
        binary_vec.insert(3);
        binary_vec.insert(7);

        assert_eq!(binary_vec.remove_item(&5), Some(5));
        assert_eq!(binary_vec.get(0), Some(&3));
        assert_eq!(binary_vec.get(1), Some(&7));
        assert_eq!(binary_vec.remove_item(&10), None); // Not found
    }

    #[test]
    fn test_get_index() {
        let mut binary_vec = BinaryVec::new();
        binary_vec.insert(5);
        binary_vec.insert(3);
        binary_vec.insert(7);

        assert_eq!(binary_vec.get_index(&5), Some(1));
        assert_eq!(binary_vec.get_index(&3), Some(0));
        assert_eq!(binary_vec.get_index(&7), Some(2));
        assert_eq!(binary_vec.get_index(&10), None); // Not found
    }
}
