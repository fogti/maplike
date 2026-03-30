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

/// A keyed collection without any operations defined.
///
/// A keyed collection is just a key-value map. We however use the name
/// `KeyedCollection` instead of `Map` to distinguish maps from vectors and
/// stable vectors, which also are keyed collections but with slightly different
/// sets of operations.
pub trait KeyedCollection {
    /// Type of the keys in the keyed collection.
    type Key;
    /// Type of the values in the keyed collection.
    type Value;
}

/// Returns a reference to the value corresponding to the key.
pub trait Get<K>: KeyedCollection {
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &K) -> Option<&Self::Value>;
}

/// Set the value of an already existing element under a key.
pub trait Set<K>: KeyedCollection {
    /// Set the value of an already existing element under a key.
    fn set(&mut self, key: K, value: Self::Value);
}

/// Insert a new key-value pair into the collection at an arbitrary key.
pub trait Insert<K>: KeyedCollection {
    /// Insert a new key-value pair into the collection at an arbitrary key.
    fn insert(&mut self, key: K, value: Self::Value);
}

/// Remove an element under a key from the collection, returning the value at
/// the key if the key was previously in the map.
pub trait Remove<K>: KeyedCollection {
    /// Remove an element from the collection, returning the value at the key if
    /// the key was previously in the map.
    fn remove(&mut self, key: &K) -> Option<Self::Value>;
}

/// Removing an element under a key using [`Remove::remove()`] does not
/// invalidate any other key.
///
/// Plain vectors such as [`Vec`] cannot implement this trait because
/// removing elements invalidates keys of other elements. Some contiguous data
/// structures, such as [`stable_vec::StableVec`] and [`thunderdome::Arena`],
/// bypass this limitation by placing a tombstone element in place of the
/// removed element.
pub trait StableRemove<K>: Remove<K> {}

/// Insert a value into the collection without specifying a key, returning
/// the key that was automatically generated.
pub trait Push<K>: KeyedCollection {
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    fn push(&mut self, value: Self::Value) -> K;
}

/// Remove the last element of the collection, returning it.
///
/// If `Push` is also implemented, calling `Pop` should revert the previous
/// pushes in their reversed order.
pub trait Pop: KeyedCollection {
    /// Remove the last element of the collection, returning it.
    fn pop(&mut self) -> Option<Self::Value>;
}

/// Remove all elements from the collection.
pub trait Clear: KeyedCollection {
    /// Remove all elements from the collection.
    fn clear(&mut self);
}

/// Returns the length of the collection.
///
/// Should be only implemented for truly contiguous data structures, for which
/// it makes sense to have a `.pop()` operation. Currently [`Vec`] is the only
/// supported data structure that satisfies this property.
pub trait Len: KeyedCollection {
    /// Returns the length of the collection.
    ///
    /// Should be only implemented for truly contiguous data structures, for which
    /// it makes sense to have a `.pop()` operation. Currently [`Vec`] is the only
    /// supported data structure that satisfies this property.
    fn len(&self) -> Self::Key;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: KeyedCollection {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Value)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
}

/// A keyed collection with stable removes.
pub trait Map<K>: Get<K> + Insert<K> + StableRemove<K> {}
impl<K, T: Get<K> + Insert<K> + StableRemove<K>> Map<K> for T {}

/// A keyed collection with pushes.
pub trait Vec<K>: Get<K> + Insert<K> + Remove<K> + Push<K> {}
impl<K, T: Get<K> + Insert<K> + Remove<K> + Push<K>> Vec<K> for T {}

/// A keyed collection with stable removes and pushes.
pub trait StableVec<K>: Vec<K> + StableRemove<K> {}
impl<K, T: Vec<K> + StableRemove<K>> StableVec<K> for T {}

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
