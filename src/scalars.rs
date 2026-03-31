// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Assign, Container};

macro_rules! impl_traits_for_scalar {
    ($($t:ty),*) => {
        $(
            impl Container for $t {
                type Key = usize;
                type Value = Self;
            }

            impl Assign for $t {
                #[inline(always)]
                fn assign(&mut self, value: Self) {
                    *self = value;
                }
            }
        )*
    };
}

impl_traits_for_scalar!(i8, i16, i32, i64, i128, isize);
impl_traits_for_scalar!(u8, u16, u32, u64, u128, usize);
impl_traits_for_scalar!(f32, f64);
impl_traits_for_scalar!(char, bool, ());
