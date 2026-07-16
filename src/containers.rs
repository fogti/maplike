// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract container traits that join together multiple operations.

use core::ops::Index;

use crate::ops::{Assign, Clear, Get, Insert, Len, Pop, Push, Remove, Set};

/// Base trait for keyed collections, without any operations defined yet.
///
/// Just a key-value map without any methods yet. We however use the name
/// `Collection` instead of `Map` to distinguish maps from vectors and stable
/// vectors, which also are keyed collections but with slightly different sets
/// of operations.
pub trait Container {
    /// Type of the keys in the keyed collection.
    type Key;
    /// Type of the values in the keyed collection.
    type Value;
}

/// A single assignable value.
pub trait Scalarlike<V = Self>: Assign<V> {}
impl<V, T: Assign<V>> Scalarlike<V> for T {}

/// A keyed collection with get, set, insert, remove, clear operations.
pub trait Maplike<K>: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear
where
    for<'a> Self: Index<&'a K>,
{
}
impl<K, T: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear> Maplike<K> for T where
    for<'a> Self: Index<&'a K>
{
}

/// A map-like keyed collection whose value is the unit type, thus behaving like
/// a set.
pub trait Setlike<K>: Maplike<K, Value = ()> {}
impl<K, T: Maplike<K, Value = ()>> Setlike<K> for T {}

/// A keyed collection with get, set, len operations.
pub trait Arraylike<K>: Index<K> + Get<K> + Set<K> + Len {}
impl<K, T: Index<K> + Get<K> + Set<K> + Len> Arraylike<K> for T {}

/// An array-like keyed collection with additional push, pop, clear operations.
pub trait Veclike<K>: Index<K> + Get<K> + Set<K> + Push<K> + Pop + Clear + Len {}
impl<K, T: Arraylike<K> + Push<K> + Pop + Clear> Veclike<K> for T {}
