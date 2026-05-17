// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Modify, Push, Remove, Set};

impl<V> Container for Arena<V> {
    type Key = Index;
    type Value = V;
}

impl<V> Assign for Arena<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
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
        Arena::insert_at(self, key, value);
    }
}

impl<V> Modify<Index> for Arena<V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: Index, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(key).expect("no value under key"));
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

impl<V> Push<Index> for Arena<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> Index {
        Arena::insert(self, value)
    }
}

impl<V> Clear for Arena<V> {
    #[inline(always)]
    fn clear(&mut self) {
        Arena::clear(self);
    }
}

impl<V> IntoIter<Index> for Arena<V> {
    type IntoIter = thunderdome::iter::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> thunderdome::iter::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}
