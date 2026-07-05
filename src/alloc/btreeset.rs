// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use alloc_::collections::BTreeSet;

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Put, Remove, Set};

impl<K> Container for BTreeSet<K> {
    type Key = K;
    type Value = ();
}

impl<K> Assign for BTreeSet<K> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Ord, Q: Ord + ?Sized> Get<K, Q> for BTreeSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        BTreeSet::get(self, key).map(|_| &())
    }
}

impl<K: Ord> Set<K> for BTreeSet<K> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        BTreeSet::insert(self, key)
    }
}

impl<K: Ord> Insert<K> for BTreeSet<K> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        BTreeSet::insert(self, key)
    }
}

impl<K: Ord, Q: Ord + ?Sized> Remove<K, Q> for BTreeSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        BTreeSet::remove(self, key).then_some(())
    }
}

impl<V: Ord> Put<V> for BTreeSet<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        BTreeSet::insert(self, value);

        None
    }
}

impl<K: Ord> Clear for BTreeSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        BTreeSet::clear(self);
    }
}

pub struct MapIntoIter<K>(alloc_::collections::btree_set::IntoIter<K>);

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
