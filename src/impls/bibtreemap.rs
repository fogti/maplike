// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use bidimap::{BiBTreeMap, Overwritten};

use crate::containers::Container;
use crate::ops::{
    Assign, Clear, Get, GetByLeft, GetByRight, Insert, IntoIter, RemoveByLeft, RemoveByRight, Set,
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

impl<L: Ord, R: Ord, Q: Ord + ?Sized> Get<L, Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> GetByLeft<L, Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_left(&self, key: &Q) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> GetByRight<L, Q> for BiBTreeMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_right(&self, key: &Q) -> Option<&L> {
        BiBTreeMap::get_by_right(self, key)
    }
}

impl<L: Ord, R: Ord> Set<L> for BiBTreeMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn set(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiBTreeMap::insert(self, key, value)
    }
}

impl<L: Ord, R: Ord> Insert<L> for BiBTreeMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn insert(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiBTreeMap::insert(self, key, value)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> RemoveByLeft<L, Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_left(&mut self, key: &Q) -> Option<R> {
        BiBTreeMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> RemoveByRight<L, Q> for BiBTreeMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_right(&mut self, key: &Q) -> Option<L> {
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
