// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVecFacade;

use crate::{Clear, Collection, Get, Insert, IntoIter, Push, Remove, Set};

impl<V, C: stable_vec::core::Core<V>> Assign for StableVecFacade<V, C> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V, C: stable_vec::core::Core<V>> Collection for StableVecFacade<V, C> {
    type Key = usize;
    type Value = V;
}

impl<V, C: stable_vec::core::Core<V>> Get<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        StableVecFacade::get(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Set<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        StableVecFacade::insert(self, index, value);
    }
}

impl<V, C: stable_vec::core::Core<V>> Insert<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn insert(&mut self, index: usize, value: V) {
        self.reserve_for(index);
        StableVecFacade::insert(self, index, value);
    }
}

impl<V, C: stable_vec::core::Core<V>> Remove<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        self.get(*index)?;
        StableVecFacade::remove(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Push<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        StableVecFacade::push(self, value)
    }
}

impl<V, C: stable_vec::core::Core<V>> Clear for StableVecFacade<V, C> {
    #[inline(always)]
    fn clear(&mut self) {
        StableVecFacade::clear(self);
    }
}

impl<V, C: stable_vec::core::Core<V>> IntoIter<usize> for StableVecFacade<V, C> {
    type IntoIter = stable_vec::iter::IntoIter<V, C>;

    #[inline(always)]
    fn into_iter(self) -> stable_vec::iter::IntoIter<V, C> {
        IntoIterator::into_iter(self)
    }
}
