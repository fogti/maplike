// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use alloc_::collections::BTreeMap;

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Modify, Remove, Set};

impl<K, V> Container for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K, V> Assign for BTreeMap<K, V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Ord, Q: Ord + ?Sized, V> Get<K, Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        BTreeMap::get(self, key)
    }
}

impl<K: Ord, V> Set<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn set(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

impl<K: Ord, Q: Ord + ?Sized, V> Modify<K, Q> for BTreeMap<K, V>
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

impl<K: Ord, V> Insert<K> for BTreeMap<K, V> {
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        BTreeMap::insert(self, key, value);
    }
}

impl<K: Ord, Q: Ord + ?Sized, V> Remove<K, Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        BTreeMap::remove(self, key)
    }
}

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
