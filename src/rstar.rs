// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::{RTree, RTreeObject};

use crate::{Get, Insert, IntoIter, KeyedCollection, Remove, StableRemove};

impl<K: RTreeObject> KeyedCollection for RTree<K> {
    type Key = K;
    type Value = ();
}

impl<K: RTreeObject + PartialEq> Get<K> for RTree<K> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        RTree::contains(self, key).then_some(&())
    }
}

impl<K: RTreeObject> Insert<K> for RTree<K> {
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

impl<K: RTreeObject + PartialEq> StableRemove<K> for RTree<K> {}

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
