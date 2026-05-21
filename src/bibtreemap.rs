// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bidimap::BiBTreeMap;

use crate::{
    Assign, Clear, Container, Get, GetByLeft, GetByRight, Insert, IntoIter, RemoveByLeft,
    RemoveByRight, Set,
};

impl<L, R> Container for BiBTreeMap<L, R> {
    type Key = L;
    type Value = R;
}

impl<L, R> Assign for BiBTreeMap<L, R> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<L: Ord, R: Ord> Get<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn get(&self, key: &L) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord> GetByLeft<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn get_by_left(&self, key: &L) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord> GetByRight<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn get_by_right(&self, key: &R) -> Option<&L> {
        BiBTreeMap::get_by_right(self, key)
    }
}

impl<L: Ord, R: Ord> Set<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn set(&mut self, key: L, value: R) {
        BiBTreeMap::insert(self, key, value);
    }
}

impl<L: Ord, R: Ord> Insert<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn insert(&mut self, key: L, value: R) {
        BiBTreeMap::insert(self, key, value);
    }
}

impl<L: Ord, R: Ord> RemoveByLeft<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn remove_by_left(&mut self, key: &L) -> Option<R> {
        BiBTreeMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Ord, R: Ord> RemoveByRight<L> for BiBTreeMap<L, R> {
    #[inline(always)]
    fn remove_by_right(&mut self, key: &R) -> Option<L> {
        BiBTreeMap::remove_by_right(self, key).map(|(key, _)| key)
    }
}

impl<L: Ord, R: Ord> Clear for BiBTreeMap<L, R> {
    #[inline(always)]
    fn clear(&mut self) {
        BiBTreeMap::clear(self);
    }
}

impl<L, R> IntoIter<L> for BiBTreeMap<L, R> {
    type IntoIter = bidimap::btree::IntoIter<L, R>;

    #[inline(always)]
    fn into_iter(self) -> bidimap::btree::IntoIter<L, R> {
        IntoIterator::into_iter(self)
    }
}
