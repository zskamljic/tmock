#[cfg(test)]
mod tests;

use std::borrow::Borrow;
use std::mem;
use std::ops::Index;

#[derive(Default)]
pub struct InsertOrderMap<K, V> {
    pairs: Vec<(K, V)>,
}

impl<K, V> InsertOrderMap<K, V>
where
    K: Eq,
{
    pub fn new() -> InsertOrderMap<K, V> {
        InsertOrderMap { pairs: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.index_of(&key) {
            Some(index) => {
                mem::replace(&mut self.pairs[index], (key, value));
            }
            None => self.pairs.push((key, value)),
        }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + PartialEq<K>,
    {
        for (value_key, value) in &self.pairs {
            if key == value_key {
                return Some(value);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn iter(&self) -> InsertOrderIteratorRef<'_, K, V> {
        InsertOrderIteratorRef {
            items: self
                .pairs
                .iter()
                .map(|entry| (&entry.0, &entry.1))
                .collect(),
        }
    }

    fn index_of(&self, key: &K) -> Option<usize> {
        for i in 0..self.pairs.len() {
            if &self.pairs[i].0 == key {
                return Some(i);
            }
        }
        None
    }
}

impl<K, V> PartialEq for InsertOrderMap<K, V>
where
    K: Eq,
    V: PartialEq,
{
    fn eq(&self, other: &InsertOrderMap<K, V>) -> bool {
        self.pairs.eq(&other.pairs)
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for InsertOrderMap<K, V>
where
    K: Eq + Borrow<Q>,
    Q: Eq + PartialEq<K>,
{
    type Output = V;

    #[inline]
    fn index(&self, index: &Q) -> &V {
        self.get(index).expect("no entry found for key")
    }
}

pub struct InsertOrderIterator<K, V> {
    items: Vec<(K, V)>,
}

impl<K, V> Iterator for InsertOrderIterator<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}

impl<K, V> IntoIterator for InsertOrderMap<K, V>
where
    K: Eq,
{
    type Item = (K, V);
    type IntoIter = InsertOrderIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        InsertOrderIterator { items: self.pairs }
    }
}

pub struct InsertOrderIteratorRef<'a, K, V> {
    items: Vec<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for InsertOrderIteratorRef<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
}

impl<'a, K, V> IntoIterator for &'a InsertOrderMap<K, V>
where
    K: Eq,
{
    type Item = (&'a K, &'a V);
    type IntoIter = InsertOrderIteratorRef<'a, K, V>;

    fn into_iter(self) -> InsertOrderIteratorRef<'a, K, V> {
        self.iter()
    }
}
