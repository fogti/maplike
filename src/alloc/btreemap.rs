// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::collections::BTreeMap;

use crate::{Clear, Get, Insert, IntoIter, KeyedCollection, Remove, Set, StableRemove};

impl<K, V> KeyedCollection for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K: Ord, V> Get<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        BTreeMap::get(self, key)
    }
}

impl<K: Ord, V> Set<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn set(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

impl<K: Ord, V> Insert<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

impl<K: Ord, V> Remove<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        BTreeMap::remove(self, key)
    }
}

impl<K: Ord, V> StableRemove<K> for BTreeMap<K, V> {}

impl<K: Ord, V> Clear for BTreeMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        BTreeMap::clear(self);
    }
}

impl<K, V> IntoIter<K> for BTreeMap<K, V> {
    type IntoIter = alloc_::collections::btree_map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> alloc_::collections::btree_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
