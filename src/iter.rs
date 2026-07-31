// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Iteration traits for map-like containers.

use crate::containers::Container;

/// Borrow the collection and yield key-value pairs.
pub trait Iter<'a, K>: Container
where
    Self: 'a,
{
    /// Iterator that borrows from the collection.
    type Iter: Iterator<Item = (K, &'a Self::Value)>;

    /// Borrow the collection and yield key-value pairs.
    fn iter(&'a self) -> Self::Iter;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: Container {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Value)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
}
