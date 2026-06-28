// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file was generated using Claude Opus 4.8 Medium with some manual
// modifications.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::Debug;

use maplike::{
    Assign, Clear, Container, Get, GetByLeft, GetByRight, Insert, IntoIter, Len, Modify, Pop, Push,
    Remove, RemoveByLeft, RemoveByRight, Set,
};

trait FromUsize {
    fn from_usize(u: usize) -> Self;
}

impl FromUsize for () {
    fn from_usize(_: usize) {}
}

impl FromUsize for usize {
    fn from_usize(u: usize) -> usize {
        u
    }
}

impl FromUsize for i32 {
    fn from_usize(u: usize) -> i32 {
        u as i32
    }
}

impl FromUsize for String {
    fn from_usize(u: usize) -> String {
        u.to_string()
    }
}

impl FromUsize for (i32, i32) {
    fn from_usize(u: usize) -> (i32, i32) {
        (u as i32, 0)
    }
}

fn check_keyed<K, V, C>(mut c: C)
where
    K: FromUsize + Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = K, Value = V> + Get<K> + Set<K> + Insert<K> + Remove<K> + Clear,
{
    let k1 = K::from_usize(1);
    let k2 = K::from_usize(2);
    let v1 = V::from_usize(10);
    let v2 = V::from_usize(20);

    assert_eq!(c.get(&k1), None);

    c.insert(k1.clone(), v1.clone());
    c.insert(k2.clone(), v2.clone());
    assert_eq!(c.get(&k1), Some(&v1));
    assert_eq!(c.get(&k2), Some(&v2));

    c.set(k1.clone(), v2.clone());
    assert_eq!(c.get(&k1), Some(&v2));

    assert_eq!(c.remove(&k1), Some(v2.clone()));
    assert_eq!(c.get(&k1), None);

    c.clear();
    assert_eq!(c.get(&k2), None);
}

fn check_modify<K, V, C>(mut c: C)
where
    K: FromUsize + Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = K, Value = V> + Insert<K> + Get<K> + Modify<K> + Clear,
{
    let k = K::from_usize(1);

    c.insert(k.clone(), V::from_usize(10));
    c.modify(&k, |v| *v = V::from_usize(99));
    assert_eq!(c.get(&k), Some(&V::from_usize(99)));

    c.clear();
    assert_eq!(c.get(&k), None);
}

fn check_into_iter<K, V, C>(mut c: C)
where
    K: FromUsize + Clone + PartialEq + Debug,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = K, Value = V> + Insert<K> + IntoIter<K>,
{
    c.insert(K::from_usize(1), V::from_usize(10));
    c.insert(K::from_usize(2), V::from_usize(20));
    c.insert(K::from_usize(3), V::from_usize(30));

    let items: Vec<(K, V)> = IntoIter::into_iter(c).collect();

    assert_eq!(items.len(), 3);
    assert!(items.contains(&(K::from_usize(1), V::from_usize(10))));
    assert!(items.contains(&(K::from_usize(2), V::from_usize(20))));
    assert!(items.contains(&(K::from_usize(3), V::from_usize(30))));
}

fn check_assign_eq<C>(initial: C, replacement: C)
where
    C: Assign + Clone + PartialEq + Debug,
{
    let mut c = initial;
    c.assign(replacement.clone());
    assert_eq!(c, replacement);
}

fn check_borrowed_str<C>(mut c: C)
where
    C: Container<Key = String, Value = i32>
        + Insert<String>
        + Get<String, str>
        + Modify<String, str>
        + Remove<String, str>,
{
    c.insert("one".to_string(), 1);
    c.insert("two".to_string(), 2);

    assert_eq!(c.get("one"), Some(&1));
    assert_eq!(c.get("missing"), None);

    c.modify("two", |v| *v = 22);
    assert_eq!(c.get("two"), Some(&22));

    assert_eq!(c.remove("one"), Some(1));
    assert_eq!(c.get("one"), None);
}

fn check_pushed<K, V, C>(mut c: C)
where
    K: Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = K, Value = V> + Push<K> + Get<K> + Set<K> + Modify<K>,
{
    let k0 = c.push(V::from_usize(10));
    let k1 = c.push(V::from_usize(20));

    assert_eq!(c.get(&k0), Some(&V::from_usize(10)));
    assert_eq!(c.get(&k1), Some(&V::from_usize(20)));

    c.set(k0.clone(), V::from_usize(11));
    assert_eq!(c.get(&k0), Some(&V::from_usize(11)));

    c.modify(&k1, |v| *v = V::from_usize(21));
    assert_eq!(c.get(&k1), Some(&V::from_usize(21)));
}

fn check_pushed_insert_remove<K, V, C>(mut c: C)
where
    K: Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = K, Value = V> + Push<K> + Get<K> + Insert<K> + Remove<K> + Clear,
{
    let k0 = c.push(V::from_usize(10));

    assert_eq!(c.remove(&k0), Some(V::from_usize(10)));
    assert_eq!(c.get(&k0), None);

    c.insert(k0.clone(), V::from_usize(15));
    assert_eq!(c.get(&k0), Some(&V::from_usize(15)));

    c.clear();
    assert_eq!(c.get(&k0), None);
}

fn check_vec<V, C>(mut c: C)
where
    V: FromUsize + Clone + PartialEq + Debug,
    C: Container<Key = usize, Value = V> + Push<usize> + Pop + Len + Clear,
{
    c.push(V::from_usize(10));
    c.push(V::from_usize(20));
    c.push(V::from_usize(30));
    assert_eq!(Len::len(&c), 3usize);

    assert_eq!(c.pop(), Some(V::from_usize(30)));
    assert_eq!(Len::len(&c), 2usize);

    c.clear();
    assert_eq!(Len::len(&c), 0usize);
}

fn check_indexed<C>(c: &mut C)
where
    C: ?Sized + Container<Key = usize, Value = i32> + Get<usize> + Set<usize> + Modify<usize> + Len,
{
    assert_eq!(Len::len(&*c), 3usize);

    assert_eq!(c.get(&0), Some(&10));

    c.set(1, 25);
    assert_eq!(c.get(&1), Some(&25));

    c.modify(&2, |v| *v += 5);
    assert_eq!(c.get(&2), Some(&35));
}

fn check_bidimap<C>(mut c: C)
where
    C: Container<Key = String, Value = i32>
        + Get<String>
        + GetByLeft<String, str>
        + GetByRight<String>
        + Set<String>
        + Insert<String>
        + RemoveByLeft<String, str>
        + RemoveByRight<String>
        + Clear,
{
    c.insert("a".to_string(), 1);
    c.insert("b".to_string(), 2);

    assert_eq!(c.get(&"a".to_string()), Some(&1));
    assert_eq!(c.get_by_left("a"), Some(&1));
    assert_eq!(c.get_by_right(&2), Some(&"b".to_string()));

    c.set("a".to_string(), 11);
    assert_eq!(c.get_by_left("a"), Some(&11));
    assert_eq!(c.get_by_right(&11), Some(&"a".to_string()));

    assert_eq!(c.remove_by_left("a"), Some(11));
    assert_eq!(c.get_by_left("a"), None);

    assert_eq!(c.remove_by_right(&2), Some("b".to_string()));
    c.clear();
    assert_eq!(c.get_by_left("b"), None);
}

#[cfg(feature = "std")]
mod hashmap {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_traits() {
        check_keyed::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_modify::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_into_iter::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_borrowed_str(HashMap::<String, i32>::new());
        check_assign_eq(
            HashMap::from([(1usize, 1i32)]),
            HashMap::from([(2usize, 2i32), (3usize, 3i32)]),
        );
    }
}

#[cfg(feature = "std")]
mod hashset {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_traits() {
        check_keyed::<usize, (), HashSet<usize>>(HashSet::new());
        check_into_iter::<usize, (), HashSet<usize>>(HashSet::new());
        check_assign_eq(HashSet::from([1usize]), HashSet::from([2usize, 3usize]));
    }
}

#[cfg(feature = "alloc")]
mod btreemap {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_traits() {
        check_keyed::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_modify::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_into_iter::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_borrowed_str(BTreeMap::<String, i32>::new());
        check_assign_eq(
            BTreeMap::from([(1usize, 1i32)]),
            BTreeMap::from([(2usize, 2i32), (3usize, 3i32)]),
        );
    }
}

#[cfg(feature = "alloc")]
mod btreeset {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_traits() {
        check_keyed::<usize, (), BTreeSet<usize>>(BTreeSet::new());
        check_into_iter::<usize, (), BTreeSet<usize>>(BTreeSet::new());
        check_assign_eq(BTreeSet::from([1usize]), BTreeSet::from([2usize, 3usize]));
    }
}

#[cfg(feature = "alloc")]
mod vec {
    use super::*;

    #[test]
    fn test_traits() {
        check_pushed::<usize, i32, Vec<i32>>(Vec::new());
        check_vec::<i32, Vec<i32>>(Vec::new());
        check_assign_eq(vec![1i32], vec![2i32, 3i32]);

        let mut c: Vec<i32> = Vec::new();
        c.push(10);
        c.push(20);
        c.push(30);
        let items: Vec<(usize, i32)> = IntoIter::into_iter(c).collect();
        assert_eq!(items, vec![(0, 10), (1, 20), (2, 30)]);
    }
}

mod array {
    use super::*;

    #[test]
    fn test_traits() {
        check_indexed(&mut [10i32, 20, 30]);
        check_assign_eq([0i32, 0, 0], [1i32, 2, 3]);
    }
}

mod slice {
    use super::*;

    #[test]
    fn test_traits() {
        let mut backing = [10i32, 20, 30];
        check_indexed(&mut backing[..]);
    }
}

mod tuple {
    use super::*;

    #[test]
    fn test_traits() {
        check_assign_eq((0i32, 0i32), (1i32, 2i32));
    }
}

#[cfg(feature = "stable-vec")]
mod stable_vec_tests {
    use super::*;
    use stable_vec::StableVec;

    #[test]
    fn test_traits() {
        check_keyed::<usize, i32, StableVec<i32>>(StableVec::new());
        check_modify::<usize, i32, StableVec<i32>>(StableVec::new());
        check_into_iter::<usize, i32, StableVec<i32>>(StableVec::new());
        check_pushed::<usize, i32, StableVec<i32>>(StableVec::new());
        check_pushed_insert_remove::<usize, i32, StableVec<i32>>(StableVec::new());

        let mut a = StableVec::new();
        a.push(1i32);
        let mut b = StableVec::new();
        b.push(2i32);
        b.push(3i32);
        check_assign_eq(a, b);
    }
}

#[cfg(feature = "thunderdome")]
mod thunderdome_tests {
    use super::*;
    use thunderdome::Arena;

    #[test]
    fn test_traits() {
        check_pushed::<thunderdome::Index, i32, Arena<i32>>(Arena::new());
        check_pushed_insert_remove::<thunderdome::Index, i32, Arena<i32>>(Arena::new());

        let mut a: Arena<i32> = Arena::new();
        a.push(1);
        a.push(2);
        a.push(3);
        let items: Vec<(thunderdome::Index, i32)> = IntoIter::into_iter(a).collect();
        assert_eq!(items.len(), 3);

        let mut x: Arena<i32> = Arena::new();
        x.push(5);
        let mut y: Arena<i32> = Arena::new();
        let j = y.push(7);
        x.assign(y);
        assert_eq!(Get::get(&x, &j), Some(&7));
    }
}

#[cfg(feature = "rstar")]
mod rstar_tests {
    use super::*;
    use rstar::RTree;

    #[test]
    fn test_traits() {
        check_keyed::<(i32, i32), (), RTree<(i32, i32)>>(RTree::new());
        check_into_iter::<(i32, i32), (), RTree<(i32, i32)>>(RTree::new());

        let mut r: RTree<(i32, i32)> = RTree::new();
        Insert::insert(&mut r, (1, 0), ());
        let mut s: RTree<(i32, i32)> = RTree::new();
        Insert::insert(&mut s, (2, 0), ());
        r.assign(s);
        assert!(Get::get(&r, &(2, 0)).is_some());
        assert!(Get::get(&r, &(1, 0)).is_none());
    }
}

#[cfg(feature = "bidimap")]
mod bibtreemap {
    use super::*;
    use bidimap::BiBTreeMap;

    #[test]
    fn test_traits() {
        check_bidimap(BiBTreeMap::<String, i32>::new());
        check_into_iter::<usize, i32, BiBTreeMap<usize, i32>>(BiBTreeMap::new());

        let mut a = BiBTreeMap::new();
        a.insert("a".to_string(), 1i32);
        let mut b = BiBTreeMap::new();
        b.insert("b".to_string(), 2i32);
        check_assign_eq(a, b);
    }
}

#[cfg(all(feature = "bidimap", feature = "std"))]
mod bihashmap {
    use super::*;
    use bidimap::BiHashMap;

    #[test]
    fn test_traits() {
        check_bidimap(BiHashMap::<String, i32>::new());
        check_into_iter::<usize, i32, BiHashMap<usize, i32>>(BiHashMap::new());

        let mut a = BiHashMap::new();
        a.insert("a".to_string(), 1i32);
        let mut b = BiHashMap::new();
        b.insert("b".to_string(), 2i32);
        check_assign_eq(a, b);
    }
}
