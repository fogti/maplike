// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use bidimap::BiHashMap;
use std_::hash::Hash;

use crate::{
    Assign, Clear, Container, Get, GetByLeft, GetByRight, Insert, IntoIter, RemoveByLeft,
    RemoveByRight, Set,
};

impl<L, R> Container for BiHashMap<L, R> {
    type Key = L;
    type Value = R;
}

impl<L, R> Assign for BiHashMap<L, R> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> Get<L, Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> GetByLeft<L, Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_left(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> GetByRight<L, Q> for BiHashMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_right(&self, key: &Q) -> Option<&L> {
        BiHashMap::get_by_right(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Set<L> for BiHashMap<L, R> {
    #[inline(always)]
    fn set(&mut self, key: L, value: R) {
        BiHashMap::insert(self, key, value);
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Insert<L> for BiHashMap<L, R> {
    #[inline(always)]
    fn insert(&mut self, key: L, value: R) {
        BiHashMap::insert(self, key, value);
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> RemoveByLeft<L, Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_left(&mut self, key: &Q) -> Option<R> {
        BiHashMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> RemoveByRight<L, Q> for BiHashMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_right(&mut self, key: &Q) -> Option<L> {
        BiHashMap::remove_by_right(self, key).map(|(key, _)| key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Clear for BiHashMap<L, R> {
    #[inline(always)]
    fn clear(&mut self) {
        BiHashMap::clear(self);
    }
}

impl<L, R> IntoIter<L> for BiHashMap<L, R> {
    type IntoIter = bidimap::hash::IntoIter<L, R>;

    #[inline(always)]
    fn into_iter(self) -> bidimap::hash::IntoIter<L, R> {
        IntoIterator::into_iter(self)
    }
}
