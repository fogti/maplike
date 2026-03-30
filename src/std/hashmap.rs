// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std_::{collections::HashMap, hash::Hash};

use crate::{Clear, Get, Insert, IntoIter, KeyedCollection, Remove, Set, StableRemove};

impl<K, V> KeyedCollection for HashMap<K, V> {
    type Key = K;
    type Value = V;
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

impl<K: Eq + Hash, V> StableRemove<K> for HashMap<K, V> {}

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
