// SPDX-FileCopyrightText: 2025 maplike contributors
// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _std::{collections::HashSet, hash::Hash};

use crate::{Get, Insert, IntoIter, Keyed, Map, Remove, StableRemove};

impl<K> Map for HashSet<K> {
    type Item = ();
}

impl<K> Keyed for HashSet<K> {
    type Key = K;
}

impl<K: Eq + Hash> Get<K> for HashSet<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        HashSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash> Insert<K> for HashSet<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        HashSet::insert(self, key);
    }
}

impl<K: Eq + Hash> Remove<K> for HashSet<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        HashSet::remove(self, key).then_some(())
    }
}

impl<K: Eq + Hash> StableRemove<K> for HashSet<K> {}

pub struct MapIntoIter<K>(_std::collections::hash_set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for HashSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
