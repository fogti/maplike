// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod compounds;
mod option;
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

#[cfg(feature = "indexmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap")))]
mod indexmap;

#[cfg(feature = "indexmap")]
#[cfg_attr(docsrs, doc(cfg(feature = "indexmap")))]
mod indexset;

#[cfg(feature = "rstar")]
#[cfg_attr(docsrs, doc(cfg(feature = "rstar")))]
mod rstar;

#[cfg(feature = "stable-vec")]
#[cfg_attr(docsrs, doc(cfg(feature = "stable-vec")))]
mod stable_vec;

#[cfg(feature = "thunderdome")]
#[cfg_attr(docsrs, doc(cfg(feature = "thunderdome")))]
mod thunderdome;

#[cfg(feature = "arrayvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "arrayvec")))]
mod arrayvec;

#[cfg(feature = "arrayvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "arrayvec")))]
mod arraystring;

#[cfg(feature = "smallvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "smallvec")))]
mod smallvec;

#[cfg(feature = "tinyvec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tinyvec")))]
mod tinyvec;
