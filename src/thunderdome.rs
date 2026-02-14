// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};

use crate::{Get, Insert, IntoIter, KeyedCollection, Push, Remove, StableRemove};

impl<V> KeyedCollection for Arena<V> {
    type Key = Index;
    type Value = V;
}

impl<V> Get<Index> for Arena<V> {
    #[inline(always)]
    fn get(&self, key: &Index) -> Option<&V> {
        Arena::get(self, *key)
    }
}

impl<V> Set<Index> for Arena<V> {
    #[inline(always)]
    fn set(&mut self, key: Index, value: V) {
        self.insert(key, value)
    }
}

impl<V> Insert<Index> for Arena<V> {
    #[inline(always)]
    fn insert(&mut self, key: Index, value: V) {
        Arena::insert_at(self, key, value);
    }
}

impl<V> Remove<Index> for Arena<V> {
    #[inline(always)]
    fn remove(&mut self, key: &Index) -> Option<V> {
        Arena::remove(self, *key)
    }
}

impl<V> StableRemove<Index> for Arena<V> {}

impl<V> Push<Index> for Arena<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> Index {
        Arena::insert(self, value)
    }
}

impl<V> IntoIter<Index> for Arena<V> {
    type IntoIter = thunderdome::iter::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> thunderdome::iter::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}
