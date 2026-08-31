// SPDX-FileCopyrightText: 2026 maplike contributors
// SPDX-FileCopyrightText: 2026 polygon_unionfind contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//! `Tracks`, a non-empty list

use crate::{
    containers::Container,
    ops::{Assign, Get, Len, Modify, Set, WithOne},
};
use core::{
    iter::{Chain, Once},
    ops::Deref,
};

/// `Tracks`, a primary/main track with an associated container of other tracks
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tracks<S, C> {
    /// The main element
    pub primary: S,

    /// The other elements
    pub sidetracks: C,
}

impl<S, C> Tracks<S, C> {
    /// Creates a new `Tracks` collection
    #[inline(always)]
    pub const fn new(primary: S, sidetracks: C) -> Self {
        Self {
            primary,
            sidetracks,
        }
    }

    /// Returns a reference to the main element
    #[inline(always)]
    pub const fn primary(&self) -> &S {
        &self.primary
    }

    /// Returns a reference to the main element
    // This function exists for uniformity with `Vec`.
    #[inline(always)]
    pub const fn first(&self) -> &S {
        &self.primary
    }

    /// Returns a reference to the tail
    #[inline(always)]
    pub const fn sidetracks(&self) -> &C {
        &self.sidetracks
    }
}

impl<S, C> Tracks<S, C> {
    /// Creates a borrowed version of this non-empty list
    #[inline]
    pub fn as_ref<'s, C2>(&'s self) -> Tracks<&'s S, C2>
    where
        &'s C: IntoIterator<Item = &'s S>,
        C2: FromIterator<&'s S>,
    {
        Tracks {
            primary: &self.primary,
            sidetracks: (&self.sidetracks).into_iter().collect(),
        }
    }

    /// Creates an unborrowed version of this non-empty list
    #[inline(always)]
    pub fn as_deref<'s, C2>(&'s self) -> Tracks<&'s <S as Deref>::Target, C2>
    where
        S: Deref,
        &'s C: IntoIterator<Item = &'s S>,
        C2: FromIterator<&'s <S as Deref>::Target>,
    {
        self.map_ref(Deref::deref)
    }

    /// Maps `Tracks<S>` to `Tracks<T>` by applying a function to the contained values.
    #[inline]
    pub fn map<T, C2, F>(self, mut f: F) -> Tracks<T, C2>
    where
        F: FnMut(S) -> T,
        C: IntoIterator<Item = S>,
        C2: FromIterator<T>,
    {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        Tracks {
            primary: f(primary),
            sidetracks: sidetracks.into_iter().map(f).collect(),
        }
    }

    /// Maps `Tracks<S>` to `Tracks<T>` by applying a function to references of the contained values.
    ///
    /// Equivalent to `self.as_ref().map(f)`, but potentially faster.
    #[inline]
    pub fn map_ref<'s, T, C2, F>(&'s self, mut f: F) -> Tracks<T, C2>
    where
        F: FnMut(&'s S) -> T,
        &'s C: IntoIterator<Item = &'s S>,
        C2: FromIterator<T>,
    {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        Tracks {
            primary: f(primary),
            sidetracks: sidetracks.into_iter().map(f).collect(),
        }
    }

    /// Applies a modification in-place, and returns `Tracks<T>` of the return values.
    #[inline]
    pub fn modify_inplace<'s, T, C2, F>(&'s mut self, mut f: F) -> Tracks<T, C2>
    where
        F: FnMut(&'s mut S) -> T,
        &'s mut C: IntoIterator<Item = &'s mut S>,
        C2: FromIterator<T>,
    {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        Tracks {
            primary: f(primary),
            sidetracks: sidetracks.into_iter().map(f).collect(),
        }
    }
}

impl<C: Container> Container for Tracks<C::Value, C> {
    type Key = Option<C::Key>;
    type Value = C::Value;
}

impl<C> WithOne<C::Value> for Tracks<C::Value, C>
where
    C: Container + Default,
{
    #[inline(always)]
    fn with_one(primary: C::Value) -> Self {
        Self {
            primary,
            sidetracks: Default::default(),
        }
    }
}

impl<C: Container> Assign for Tracks<C::Value, C> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<C, K> Get<Option<K>> for Tracks<C::Value, C>
where
    C: Container + Get<K>,
{
    #[inline]
    fn get(&self, index: &Option<K>) -> Option<&C::Value> {
        if let Some(index_prev) = index {
            self.sidetracks.get(index_prev)
        } else {
            Some(&self.primary)
        }
    }
}

impl<C, K> Set<Option<K>> for Tracks<<C as Container>::Value, C>
where
    C: Container + Set<K>,
    <C as Set<K>>::Output: From<C::Value>,
{
    type Output = <C as Set<K>>::Output;

    #[inline]
    fn set(&mut self, index: Option<K>, value: C::Value) -> <C as Set<K>>::Output {
        use core::mem::replace;
        match index {
            None => replace(&mut self.primary, value).into(),
            Some(index_prev) => self.sidetracks.set(index_prev, value),
        }
    }
}

impl<C, K> Modify<Option<K>> for Tracks<C::Value, C>
where
    C: Container + Modify<K>,
{
    #[inline]
    fn modify<F>(&mut self, index: &Option<K>, mut f: F)
    where
        F: for<'a> FnMut(&'a mut C::Value),
    {
        match index {
            None => f(&mut self.primary),
            Some(index_prev) => self.sidetracks.modify(index_prev, f),
        }
    }
}

impl<C: Len> Len for Tracks<C::Value, C> {
    #[inline(always)]
    fn len(&self) -> usize {
        1 + self.sidetracks.len()
    }
}

impl<C> IntoIterator for Tracks<C::Item, C>
where
    C: IntoIterator,
{
    type Item = C::Item;
    type IntoIter = Chain<Once<C::Item>, C::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        core::iter::once(primary).chain(sidetracks)
    }
}

impl<'a, S, C> IntoIterator for &'a Tracks<S, C>
where
    &'a C: IntoIterator<Item = &'a S>,
{
    type Item = &'a S;
    type IntoIter = Chain<Once<&'a S>, <&'a C as IntoIterator>::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        core::iter::once(primary).chain(sidetracks)
    }
}

impl<'a, S, C> IntoIterator for &'a mut Tracks<S, C>
where
    &'a mut C: IntoIterator<Item = &'a mut S>,
{
    type Item = &'a mut S;
    type IntoIter = Chain<Once<&'a mut S>, <&'a mut C as IntoIterator>::IntoIter>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks {
            primary,
            sidetracks,
        } = self;
        core::iter::once(primary).chain(sidetracks)
    }
}
