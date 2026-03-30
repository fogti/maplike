// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/maplike")]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std as std_;

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc as alloc_;

/// Replace self with a new value.
///
/// This is mainly useful for scalars: these do not have get, set, insert,
/// remove, push, pop, clear, len operations, but may still be assigned to.
pub trait Assign {
    /// Assign a new value to `*self`.
    fn assign(&mut self, value: Self);
}

/// Base trait for keyed collections, without any operations defined yet.
///
/// Just a key-value map without any methods yet. We however use the name
/// `Collection` instead of `Map` to distinguish maps from vectors and stable
/// vectors, which also are keyed collections but with slightly different sets
/// of operations.
pub trait Collection {
    /// Type of the keys in the keyed collection.
    type Key;
    /// Type of the values in the keyed collection.
    type Value;
}

/// Returns a reference to the value corresponding to the key.
pub trait Get<K>: Collection {
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &K) -> Option<&Self::Value>;
}

/// Set the value of an already existing element under a key.
pub trait Set<K>: Collection {
    /// Set the value of an already existing element under a key.
    fn set(&mut self, key: K, value: Self::Value);
}

/// Insert a new key-value pair into the collection at an arbitrary key.
pub trait Insert<K>: Collection {
    /// Insert a new key-value pair into the collection at an arbitrary key.
    fn insert(&mut self, key: K, value: Self::Value);
}

/// Remove an element under a key from the collection, returning the value
/// at the key if the key was previously in the map. Other keys are not
/// invalidated.
///
/// `Vec` obviously does not implement this trait because its element
/// removal methods, [`Vec::insert()`] and [`Vec::swap_insert()`], invalidate
/// existing indices.
///
/// If you need this trait on a contiguous data type with constant-time
/// insertion, lookup, and removal, try [`stable_vec::StableVec`] or
/// [`thunderdome::Arena`].
pub trait Remove<K>: Collection {
    /// Remove an element under a key from the collection, returning the value
    /// at the key if the key was previously in the map. Other keys are not
    /// invalidated.
    fn remove(&mut self, key: &K) -> Option<Self::Value>;
}

/// Insert a value into the collection without specifying a key, returning
/// the key that was automatically generated.
pub trait Push<K>: Collection {
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    fn push(&mut self, value: Self::Value) -> K;
}

/// Remove the last element of the collection, returning it.
///
/// If `Push` is also implemented, calling `Pop` should revert the previous
/// pushes in their reversed order.
pub trait Pop: Collection {
    /// Remove the last element of the collection, returning it.
    fn pop(&mut self) -> Option<Self::Value>;
}

/// Remove all elements from the collection.
pub trait Clear: Collection {
    /// Remove all elements from the collection.
    fn clear(&mut self);
}

/// Returns the length of the collection.
///
/// Should be only implemented for truly contiguous data structures, for which
/// it makes sense to have a `.pop()` operation. Currently [`Vec`] is the only
/// supported data structure that satisfies this property.
pub trait Len: Collection {
    /// Returns the length of the collection.
    ///
    /// Should be only implemented for truly contiguous data structures, for which
    /// it makes sense to have a `.pop()` operation. Currently [`Vec`] is the only
    /// supported data structure that satisfies this property.
    fn len(&self) -> Self::Key;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: Collection {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Value)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
}

/// A keyed collection with get, set, insert, remove, clear operations.
pub trait Maplike<K>: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear {}
impl<K, T: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear> Maplike<K> for T {}

/// A map-like whose value is the unit type, thus behaving like a set.
pub trait Setlike<K>: Maplike<K, Value = ()> {}
impl<K, T: Maplike<K, Value = ()>> Setlike<K> for T {}

/// A keyed collection with get, set, push, pop, clear, len operations.
pub trait Veclike<K>: Get<K> + Set<K> + Push<K> + Pop + Clear + Len {}
impl<K, T: Get<K> + Set<K> + Push<K> + Pop + Clear + Len> Veclike<K> for T {}

#[cfg(feature = "std")]
mod std;

// No feature for alloc because it would be always enabled anyway.
mod alloc;

#[cfg(feature = "stable-vec")]
mod stable_vec;

#[cfg(feature = "thunderdome")]
mod thunderdome;

#[cfg(feature = "rstar")]
mod rstar;
