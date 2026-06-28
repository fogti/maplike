// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use std_::{collections::HashSet, hash::Hash};

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Remove, Set};

impl<K> Container for HashSet<K> {
    type Key = K;
    type Value = ();
}

impl<K> Assign for HashSet<K> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Get<K, Q> for HashSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        HashSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash> Set<K> for HashSet<K> {
    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) {
        HashSet::insert(self, key);
    }
}

impl<K: Eq + Hash> Insert<K> for HashSet<K> {
    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        HashSet::insert(self, key);
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Remove<K, Q> for HashSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        HashSet::remove(self, key).then_some(())
    }
}

impl<K: Eq + Hash> Clear for HashSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        HashSet::clear(self);
    }
}

pub struct MapIntoIter<K>(std_::collections::hash_set::IntoIter<K>);

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
