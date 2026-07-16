// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::Hash;

use indexmap::IndexMap;

use crate::containers::Container;
use crate::ops::{Assign, Clear, Get, Insert, IntoIter, Modify, Remove, Set};

impl<K, V> Container for IndexMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K, V> Assign for IndexMap<K, V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V> Get<K, Q> for IndexMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        IndexMap::get(self, key)
    }
}

impl<K: Eq + Hash, V> Set<K> for IndexMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        IndexMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V> Modify<K, Q> for IndexMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn modify<F>(&mut self, key: &Q, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(key).expect("no value under key"));
    }
}

impl<K: Eq + Hash, V> Insert<K> for IndexMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        IndexMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V> Remove<K, Q> for IndexMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        IndexMap::shift_remove(self, key)
    }
}

impl<K: Eq + Hash, V> Clear for IndexMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexMap::clear(self);
    }
}

impl<K, V> IntoIter<K> for IndexMap<K, V> {
    type IntoIter = indexmap::map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> indexmap::map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
