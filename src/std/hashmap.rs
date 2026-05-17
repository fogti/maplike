// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std_::{collections::HashMap, hash::Hash};

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Modify, Remove, Set};

impl<K, V> Container for HashMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K, V> Assign for HashMap<K, V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Eq + Hash, V> Get<K> for HashMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
}

impl<K: Eq + Hash, V> Set<K> for HashMap<K, V> {
    #[inline(always)]
    fn set(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }
}

impl<K: Eq + Hash, V> Modify<K> for HashMap<K, V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(&key).expect("no value under key"));
    }
}

impl<K: Eq + Hash, V> Insert<K> for HashMap<K, V> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        HashMap::insert(self, key, value);
    }
}

impl<K: Eq + Hash, V> Remove<K> for HashMap<K, V> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        HashMap::remove(self, key)
    }
}

impl<K: Eq + Hash, V> Clear for HashMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        HashMap::clear(self);
    }
}

impl<K, V> IntoIter<K> for HashMap<K, V> {
    type IntoIter = std_::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> std_::collections::hash_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
