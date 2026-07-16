// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::rc::{Rc, Weak};

use crate::{Assign, Clear, Container, Get, IntoIter, Len, Modify, Put, Remove, Set, WithOne};

impl<V> Container for Rc<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Rc<V> {
    #[inline(always)]
    fn with_one(value: V) -> Self {
        Rc::new(value)
    }
}

impl<V> Assign for Rc<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for Rc<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { Some(&**self) } else { None }
    }
}

impl<V> Set<usize> for Rc<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        Some(core::mem::replace(
            Rc::get_mut(self).expect("Rc is not uniquely owned"),
            value,
        ))
    }
}

impl<V> Modify<usize> for Rc<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        assert_eq!(*index, 0);
        f(Rc::get_mut(self).expect("Rc is not uniquely owned"))
    }
}

impl<V> Put<V> for Rc<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Some(core::mem::replace(
            Rc::get_mut(self).expect("Rc is not uniquely owned"),
            value,
        ))
    }
}

impl<V> Len for Rc<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        1
    }
}

impl<V> IntoIter<usize> for Rc<V> {
    type IntoIter = core::iter::Enumerate<core::iter::Once<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(match Rc::try_unwrap(self) {
            Ok(value) => value,
            Err(_) => panic!("Rc is not uniquely owned"),
        })
        .enumerate()
    }
}

impl<V> Container for Weak<V> {
    type Key = usize;
    type Value = V;
}

impl<V> Assign for Weak<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Remove<usize> for Weak<V> {
    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        if *index == 0 {
            *self = Weak::new();
            None
        } else {
            None
        }
    }
}

impl<V> Clear for Weak<V> {
    #[inline(always)]
    fn clear(&mut self) {
        *self = Weak::new();
    }
}

impl<V> Len for Weak<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        usize::from(self.upgrade().is_some())
    }
}
