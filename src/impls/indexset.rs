// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::Hash;

use indexmap::IndexSet;

use crate::containers::Container;
use crate::iter::IntoIter;
use crate::ops::{Assign, Clear, Get, Insert, Put, Remove, Set, WithOne};

impl<K> Container for IndexSet<K> {
    type Key = K;
    type Value = ();
}

impl<K: Eq + Hash> WithOne<K> for IndexSet<K> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut indexset = IndexSet::new();
        IndexSet::insert(&mut indexset, element);

        indexset
    }
}

impl<K> Assign for IndexSet<K> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Get<K, Q> for IndexSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        IndexSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash> Set<K> for IndexSet<K> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        IndexSet::insert(self, key)
    }
}

impl<K: Eq + Hash> Insert<K> for IndexSet<K> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        IndexSet::insert(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Remove<K, Q> for IndexSet<K>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        IndexSet::shift_remove(self, key).then_some(())
    }
}

impl<V: Eq + Hash> Put<V> for IndexSet<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        IndexSet::insert(self, value);

        None
    }
}

impl<K: Eq + Hash> Clear for IndexSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexSet::clear(self);
    }
}

pub struct MapIntoIter<K>(indexmap::set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for IndexSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
