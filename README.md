<!--
SPDX-FileCopyrightText: 2026 maplike contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![CI Status](https://ci.codeberg.org/api/badges/15907/status.svg)](https://ci.codeberg.org/repos/15907)
[![Docs](https://docs.rs/maplike/badge.svg)](https://docs.rs/maplike/)
[![Crates.io](https://img.shields.io/crates/v/maplike.svg)](https://crates.io/crates/maplike)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/maplike.svg)](#licence)

# maplike

This crate provides traits for common operations over map-like, set-like, and
vec-like data structures: get, set, insert, remove, push, pop, clear, len.

## Supported collections

### Standard library

Rust's standard library collections are supported via built-in convenience
implementations:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html), gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html), not feature-gated;
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), not feature-gated, but does not support stable removal.

### Third-party types

In addition to the standard library, `maplike` has built-in feature-gated
convenience implementations for data structures from certain external crates:

- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature; (example usage:
  [examples/stable_vec.rs](./examples/stable_vec.rs)),
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature; (example usage:
  [examples/thunderdome.rs](./examples/thunderdome.rs)),
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), gated by the
  `rstar` feature. (example usage: [examples/rstar.rs](./examples/rstar.rs)).
- [`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/), gated by the
  `rstared` feature.

## Unsupported collections

Standard library's `VecDeque` is unsupported.

Among stable vector data structures,
[`Slab`](https://docs.rs/slab/latest/slab/),
[`SlotMap`](https://docs.rs/slotmap/latest/slotmap/),
[`generational-arena`](https://docs.rs/generational-arena/latest/generational_arena/)
cannot be supported because they lack interfaces for insertion at an arbitrary
key.

### Technical sidenotes

Unlike maps and sets, not all stable vector data
structures allow insertion and removal at arbitrary indexes regardless of
whether they are vacant, occupied or out of bounds. For `StableVec`, we managed
to implement inserting at out-of-bound indexes by changing the length before
insertion using the
[`.reserve_for()`](https://docs.rs/stable-vec/latest/stable_vec/struct.StableVecFacade.html#method.reserve_for)
method. For `thunderdome::Arena`, we insert at arbitrary key directly via the
[`.insert_at()`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.insert_at)
method. Collections for which we could not achieve this are documented in the
section below.

For `Slab`, an interface to insert at an arbitrary key is missing apparently
[because](https://github.com/tokio-rs/slab/issues/117#issuecomment-1159741097)
the [freelist](https://en.wikipedia.org/wiki/Free_list) `Slab` uses to keep
track of its vacant indexes is only singly-linked, not doubly-linked. Inserting
an element at an arbitrary vacant index would require removing that index from
the freelist. But since there is no backwards link available at a given key,
doing so would require traversing the freelist from the beginning to find the
position of the previous node, which would incur an overly slow `O(n)` time
cost.
