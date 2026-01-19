// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/maplike")]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std as _std;

// No feature for `alloc` because it would be always enabled anyway.
extern crate alloc as _alloc;

/// A map-like collection that stores values of type `Item`.
pub trait Map {
    /// Type of the items in the map.
    type Item;
}

/// A keyed map with key of type `Key`.
///
/// This trait is needed to prevent rustc error E0207:
///
/// error[E0207]: the type parameter (...) is not constrained by the impl trait, self type, or predicates
///
/// by consuming the type parameter into the associated type `Key`.
pub trait Keyed {
    /// Type of the keys in the map.
    type Key;
}

/// Returns a reference to the value corresponding to the key.
pub trait Get<K>: Map {
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &K) -> Option<&Self::Item>;
}

/// Insert a key-value pair into the collection.
pub trait Insert<K>: Map {
    /// Insert a key-value pair into the collection.
    fn insert(&mut self, key: K, value: Self::Item);
}

/// Remove an element under a key from the collection, returning the value at
/// the key if the key was previously in the map.
pub trait Remove<K>: Map {
    /// Remove a key from the collection, returning the value at the key if the
    /// key was previously in the map.
    fn remove(&mut self, key: &K) -> Option<Self::Item>;
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
pub trait Push<K>: Map {
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    fn push(&mut self, value: Self::Item) -> K;
}

/// Remove the last element of the collection, returning it.
///
/// If `Push` is also implemented, calling `Pop` should revert the previous
/// pushes in their reversed order.
pub trait Pop: Map {
    /// Remove the last element of the collection, returning it.
    fn pop(&mut self) -> Option<Self::Item>;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: Map + Keyed {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Item)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
}

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
