// SPDX-FileCopyrightText: 2026 maplike contributors
// SPDX-FileCopyrightText: 2026 polygon_unionfind contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//! `Tracks`, a non-empty list

use crate::{
    containers::Container,
    ops::{Assign, Get, Len, Modify, Set, WithOne},
};
use alloc_::{boxed::Box, vec::Vec};
use core::{
    iter::{Chain, Once},
    ops::Deref,
};

/// `Tracks`, a non-empty list.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tracks<S> {
    /// The main element
    pub primary: S,

    /// The other elements
    pub parallels: Box<[S]>,
}

impl<S> Tracks<S> {
    /// Creates a new non-empty list
    #[inline(always)]
    pub const fn new(primary: S, parallels: Box<[S]>) -> Self {
        Self { primary, parallels }
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
    pub const fn parallels(&self) -> &[S] {
        &self.parallels
    }

    /// Creates a borrowed version of this non-empty list
    #[inline]
    pub fn as_ref(&self) -> Tracks<&S> {
        Tracks {
            primary: &self.primary,
            parallels: self.parallels.iter().collect(),
        }
    }

    /// Creates an unborrowed version of this non-empty list
    #[inline(always)]
    pub fn as_deref(&self) -> Tracks<&<S as Deref>::Target>
    where
        S: Deref,
    {
        self.map_ref(Deref::deref)
    }

    /// Maps `Tracks<S>` to `Tracks<T>` by applying a function to the contained values.
    #[inline]
    pub fn map<T, F>(self, mut f: F) -> Tracks<T>
    where
        F: FnMut(S) -> T,
    {
        let Tracks { primary, parallels } = self;
        Tracks {
            primary: f(primary),
            parallels: Vec::from(parallels).into_iter().map(f).collect(),
        }
    }

    /// Maps `Tracks<S>` to `Tracks<T>` by applying a function to references of the contained values.
    ///
    /// Equivalent to `self.as_ref().map(f)`, but potentially faster.
    #[inline]
    pub fn map_ref<'s, T, F>(&'s self, mut f: F) -> Tracks<T>
    where
        F: FnMut(&'s S) -> T,
    {
        let Tracks { primary, parallels } = self;
        Tracks {
            primary: f(primary),
            parallels: parallels.iter().map(f).collect(),
        }
    }

    /// Applies a modification in-place, and returns `Tracks<T>` of the return values.
    #[inline]
    pub fn modify_inplace<'s, T, F>(&'s mut self, mut f: F) -> Tracks<T>
    where
        F: FnMut(&'s mut S) -> T,
    {
        let Tracks { primary, parallels } = self;
        Tracks {
            primary: f(primary),
            parallels: parallels.iter_mut().map(f).collect(),
        }
    }
}

impl<S> Container for Tracks<S> {
    type Key = usize;
    type Value = S;
}

impl<S> WithOne<S> for Tracks<S> {
    #[inline(always)]
    fn with_one(primary: S) -> Self {
        Self {
            primary,
            parallels: Vec::new().into_boxed_slice(),
        }
    }
}

impl<S> Assign for Tracks<S> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<S> Get<usize> for Tracks<S> {
    #[inline]
    fn get(&self, index: &usize) -> Option<&S> {
        if let Some(index_prev) = (*index).checked_sub(1) {
            self.parallels.get(index_prev)
        } else {
            Some(&self.primary)
        }
    }
}

impl<S> Tracks<S> {
    /// Returns a mutable reference to the value corresponding to the key.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut S> {
        if let Some(index_prev) = index.checked_sub(1) {
            self.parallels.get_mut(index_prev)
        } else {
            Some(&mut self.primary)
        }
    }
}

impl<S> Set<usize> for Tracks<S> {
    type Output = Option<S>;

    #[inline]
    fn set(&mut self, index: usize, value: S) -> Option<S> {
        Some(core::mem::replace(self.get_mut(index)?, value))
    }
}

impl<S> Modify<usize> for Tracks<S> {
    #[inline]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut S),
    {
        f(self.get_mut(*index).unwrap())
    }
}

impl<S> Len for Tracks<S> {
    #[inline(always)]
    fn len(&self) -> usize {
        1 + self.parallels.len()
    }
}

impl<S> IntoIterator for Tracks<S> {
    type Item = S;
    type IntoIter = Chain<Once<S>, alloc_::vec::IntoIter<S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks { primary, parallels } = self;
        core::iter::once(primary).chain(Vec::from(parallels))
    }
}

impl<'a, S> IntoIterator for &'a Tracks<S> {
    type Item = &'a S;
    type IntoIter = Chain<Once<&'a S>, core::slice::Iter<'a, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks { primary, parallels } = self;
        core::iter::once(primary).chain(&parallels[..])
    }
}

impl<'a, S> IntoIterator for &'a mut Tracks<S> {
    type Item = &'a mut S;
    type IntoIter = Chain<Once<&'a mut S>, core::slice::IterMut<'a, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let Tracks { primary, parallels } = self;
        core::iter::once(primary).chain(&mut parallels[..])
    }
}
