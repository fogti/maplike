// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "derive")]

use maplike::Assign;

#[derive(Assign, Debug, PartialEq)]
struct Struct {
    integer: i64,
    string: String,
}

#[derive(Assign, Debug, PartialEq)]
enum Enum {
    Usize(usize),
    Strings(String, String),
}

#[test]
fn test_assign_on_struct() {
    let mut s = Struct {
        integer: 0,
        string: "old".to_string(),
    };

    s.assign(Struct {
        integer: 1,
        string: "new".to_string(),
    });

    assert_eq!(
        s,
        Struct {
            integer: 1,
            string: "new".to_string(),
        }
    );
}

#[test]
fn test_assign_on_enum() {
    let mut e = Enum::Usize(0);

    e.assign(Enum::Strings("string1".to_string(), "string2".to_string()));

    assert_eq!(
        e,
        Enum::Strings("string1".to_string(), "string2".to_string())
    );
}
