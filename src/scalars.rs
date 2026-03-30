// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Assign;

macro_rules! impl_assign_for_scalar {
    ($($t:ty),*) => {
        $(
            impl Assign for $t {
                #[inline(always)]
                fn assign(&mut self, value: Self) {
                    *self = value;
                }
            }
        )*
    };
}

impl_assign_for_scalar!(i8, i16, i32, i64, i128, isize);
impl_assign_for_scalar!(u8, u16, u32, u64, u128, usize);
impl_assign_for_scalar!(f32, f64);
impl_assign_for_scalar!(char, bool, ());
