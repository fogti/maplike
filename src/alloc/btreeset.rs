// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _alloc::collections::BTreeSet;

use crate::{Get, Insert, IntoIter, KeyedCollection, Remove, Set, StableRemove};

impl<K> KeyedCollection for BTreeSet<K> {
    type Key = K;
    type Value = ();
}

impl<K: Ord> Get<K> for BTreeSet<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        BTreeSet::get(self, key).map(|_| &())
    }
}

impl<K: Ord> Set<K> for BTreeSet<K> {
    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) {
        BTreeSet::insert(self, key);
    }
}

impl<K: Ord> Insert<K> for BTreeSet<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        BTreeSet::insert(self, key);
    }
}

impl<K: Ord> Remove<K> for BTreeSet<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        BTreeSet::remove(self, key).then_some(())
    }
}

impl<K: Ord> StableRemove<K> for BTreeSet<K> {}

pub struct MapIntoIter<K>(_alloc::collections::btree_set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for BTreeSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
