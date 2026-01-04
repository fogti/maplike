// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _alloc::collections::BTreeMap;

use crate::{Get, Insert, IntoIter, Keyed, Map, Remove, StableRemove};

impl<K, V> Map for BTreeMap<K, V> {
    type Item = V;
}

impl<K, V> Keyed for BTreeMap<K, V> {
    type Key = K;
}

impl<K: Ord, V> Get<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        BTreeMap::get(self, key)
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

impl<K, V> IntoIter<K> for BTreeMap<K, V> {
    type IntoIter = _alloc::collections::btree_map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> _alloc::collections::btree_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
