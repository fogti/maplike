// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/maplike")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

use core::ops::Index;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
extern crate std as std_;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
extern crate alloc as alloc_;

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use maplike_derive::{Assign, Container};

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

/// Replace self with a new value.
///
/// This is mainly useful for scalars: these do not have get, set, insert,
/// remove, push, pop, clear, len operations, but may still be assigned to.
pub trait Assign<V = Self>: Container {
    /// Replace self with a new value.
    fn assign(&mut self, value: V);
}

/// Returns a reference to the value corresponding to the key.
pub trait Get<K>: Container {
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &K) -> Option<&Self::Value>;
}

/// Returns a reference to the right value corresponding to the given left value
/// in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`GetByRight::get_by_right()`], and should
/// behave identically to [`Get`].
pub trait GetByLeft<K>: Container {
    /// Returns a reference to the right value corresponding to the given left value.
    fn get_by_left(&self, key: &K) -> Option<&Self::Value>;
}

/// Returns a reference to the left value corresponding to the given right value
/// in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`GetByRight::get_by_left()`].
///
/// Note that key and value are unusually inverted here: `Self::Value` is
/// actually the key, while `K` is the value.
pub trait GetByRight<K>: Container {
    /// Returns a reference to the right value corresponding to the given left value.
    fn get_by_right(&self, key: &Self::Value) -> Option<&K>;
}

/// Set the value of an already existing element under a key.
pub trait Set<K>: Container {
    /// Set the value of an already existing element under a key.
    fn set(&mut self, key: K, value: Self::Value);
}

/// Modify the value under key with a closure.
///
/// This is useful if something always has to be done before or after the
/// modification to maintain an invariant.
pub trait Modify<K>: Container {
    /// Modify the value under key with a closure.
    fn modify<F>(&mut self, key: K, f: F)
    where
        F: FnOnce(&mut Self::Value);
}

/// Insert a new key-value pair into the collection at an arbitrary key.
pub trait Insert<K>: Container {
    /// Insert a new key-value pair into the collection at an arbitrary key.
    fn insert(&mut self, key: K, value: Self::Value);
}

/// Remove an element under a key from the collection, returning the value
/// at the key if the key was previously in the map. Other keys are not
/// invalidated.
///
/// [`Vec`] obviously does not implement this trait because its element removal
/// methods, [`Vec::insert()`] and [`Vec::swap_insert()`], invalidate existing
/// indices.
///
/// If you need this trait on a contiguous data type with constant-time
/// insertion, lookup, and removal, try [`stable_vec::StableVec`] or
/// [`thunderdome::Arena`].
pub trait Remove<K>: Container {
    /// Remove an element under a key from the collection, returning the value
    /// at the key if the key was previously in the map. Other keys are not
    /// invalidated.
    fn remove(&mut self, key: &K) -> Option<Self::Value>;
}

/// Remove the left and right values from pair corresponding to the given left
/// value in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`RemoveByRight::remove_by_right()`], and should
/// behave identically to [`Remove`].
pub trait RemoveByLeft<K>: Container {
    /// Remove the left and right values from pair corresponding to the given
    /// left value in a bidirectional map.
    fn remove_by_left(&mut self, key: &K) -> Option<Self::Value>;
}

/// Remove the left and right values from pair corresponding to the given right
/// value in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`RemoveByLeft::remove_by_left()`].
///
/// Note that key and value are unusually inverted here: `Self::Value` is
/// actually the key, while `K` is the value.
pub trait RemoveByRight<K>: Container {
    /// Remove the left and right values from pair corresponding to the given
    /// left value in a bidirectional map.
    fn remove_by_right(&mut self, key: &Self::Value) -> Option<K>;
}

/// Insert a value into the collection without specifying a key, returning
/// the key that was automatically generated.
pub trait Push<K>: Container {
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    fn push(&mut self, value: Self::Value) -> K;
}

/// Remove the last element of the collection, returning it.
///
/// If `Push` is also implemented, calling `Pop` should revert the previous
/// pushes in their reversed order.
pub trait Pop: Container {
    /// Remove the last element of the collection, returning it.
    fn pop(&mut self) -> Option<Self::Value>;
}

/// Remove all elements from the collection.
pub trait Clear: Container {
    /// Remove all elements from the collection.
    fn clear(&mut self);
}

/// Returns the length of the collection.
///
/// Should be only implemented for truly contiguous data structures, for which
/// it makes sense to have a `.pop()` operation. Currently [`Vec`] is the only
/// supported data structure that satisfies this property.
pub trait Len: Container {
    /// Returns the length of the collection.
    fn len(&self) -> Self::Key;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: Container {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Value)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
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

mod compounds;
mod scalars;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod std;

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod alloc;

#[cfg(feature = "bidimap")]
#[cfg_attr(docsrs, doc(cfg(feature = "bidimap")))]
mod bibtreemap;

#[cfg(all(feature = "bidimap", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "bidimap", feature = "std"))))]
mod bihashmap;

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
mod rstar;

#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
mod stable_vec;

#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
mod thunderdome;
