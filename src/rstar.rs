// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::{RTree, RTreeObject};

use crate::{Assign, Clear, Container, Get, Insert, IntoIter, Remove, Set};

impl<K: RTreeObject> Container for RTree<K> {
    type Key = K;
    type Value = ();
}

impl<K: RTreeObject> Assign for RTree<K> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: RTreeObject + PartialEq> Get<K> for RTree<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        RTree::contains(self, key).then_some(&())
    }
}

impl<K: RTreeObject + PartialEq> Set<K> for RTree<K> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) {
        RTree::remove(self, &key);
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject> Insert<K> for RTree<K> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject + PartialEq> Remove<K> for RTree<K> {
    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        RTree::remove(self, key).map(|_| ())
    }
}

impl<K: RTreeObject + PartialEq> Clear for RTree<K> {
    #[inline(always)]
    fn clear(&mut self) {
        // TODO: Send a path upstream to implement `.clear()` efficiently,
        // without having to drain.
        self.drain().for_each(drop);
    }
}

pub struct MapIntoIter<K: RTreeObject>(rstar::iterators::IntoIter<K>);

impl<K: RTreeObject> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K: RTreeObject> IntoIter<K> for RTree<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
