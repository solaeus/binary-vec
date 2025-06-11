#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
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

    /// Returns an iterator over references to the items in the `BinaryVec`.
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec.iter()
    }

    /// Returns an iterator over mutable references to the items in the `BinaryVec`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.vec.iter_mut()
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

    /// Checks if the `BinaryVec` contains the specified value.
    pub fn contains(&self, value: &T) -> bool {
        self.vec.binary_search(value).is_ok()
    }

    /// Returns the number of elements that can be stored in the `BinaryVec` without reallocating.
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// Returns the number of elements in the `BinaryVec`.
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Checks if the `BinaryVec` is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Clears the `BinaryVec`, removing all elements.
    pub fn clear(&mut self) {
        self.vec.clear();
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the `BinaryVec`.
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    /// Shrinks the capacity of the `BinaryVec` to fit its current length.
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit();
    }

    /// Resizes the `BinaryVec` to contain `new_len` elements, filling new elements with `value`.
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.vec.resize(new_len, value);
    }

    /// Returns a reference to the underlying vector.
    pub fn as_slice(&self) -> &[T] {
        &self.vec
    }

    /// Returns a mutable reference to the underlying vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec
    }

    /// Returns the first element of the `BinaryVec`, or `None` if it is empty.
    pub fn first(&self) -> Option<&T> {
        self.vec.first()
    }

    /// Returns the last element of the `BinaryVec`, or `None` if it is empty.
    pub fn last(&self) -> Option<&T> {
        self.vec.last()
    }
}

impl<T: Ord> Default for BinaryVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for BinaryVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
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
