#![allow(unused_imports)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::suspicious_arithmetic_impl)]

// using Rust(1.42.0)
// My Library Repo is at "https://github.com/Loptall/prcn_lib"
// check available crates on AtCoder at "https://atcoder.jp/contests/language-test-202001"

/*
    AtCoder Language Update 2020204 お疲れ様でした! 有難いです
*/

// Dep crates are...
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::mem::swap;
use std::ops::*;
use std::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};

use itertools::*;
use itertools_num::*;
use lazy_static::lazy_static;
use maplit::{btreemap, btreeset, hashmap, hashset};
use num_bigint::{BigInt, BigUint};
use num_integer::{binomial, gcd, lcm, Integer};
use permutohedron::Heap;
use proconio::{fastout, input, is_stdin_empty, marker::*};
use rand::random;

// code...
fn main() {
    input! {
        n: usize, mut a: usize, mut b: usize, mut c: usize,
        s: [String; n]
    }

    let mut ans = "Yes";
    let mut chose = Vec::with_capacity(n);
    let mut abc = vec![a, b, c];

    for (now, next) in s.iter().tuple_windows() {
        let cln = abc.clone();
        let (first, second) = to_abc(&mut abc, now);
        if *first + *second == 0 {
            ans = "No";
            break;
        } else if *first + *second == 1 {
            if *first == 1 {
                chose.push(now.chars().nth(1).unwrap());
            } else {
                chose.push(now.chars().nth(0).unwrap());
            }
            swap(first, second);
        } else {
            if *first == 0 {
                *first += 1;
                *second -= 1;
                chose.push(now.chars().nth(0).unwrap());
            } else if *second == 0 {
                *first -= 1;
                *second += 1;
                chose.push(now.chars().nth(1).unwrap());
            } else {
                match common_abc(&now, &next, &cln) {
                    Some('A') => {
                        *first += 1;
                        *second -= 1;
                        chose.push('A');
                    }
                    Some('B') => {
                        chose.push('B');
                        if &now[..] == "AB" {
                            *first -= 1;
                            *second += 1;
                        } else {
                            *first += 1;
                            *second -= 1;
                        }
                    }
                    Some('C') => {
                        *first -= 1;
                        *second += 1;
                        chose.push('C');
                    }
                    Some(_) => unreachable!(),
                    None => {
                        ans = "No";
                        break;
                    }
                }
            }
        }
    }

    let last: (usize, usize) = s
        .last()
        .unwrap()
        .chars()
        .collect_vec()
        .into_iter()
        .map(|x: char| match x {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => unreachable!(),
        })
        .collect_tuple()
        .unwrap();

    if abc[last.0] == 0 && abc[last.1] == 0 {
        ans = "No";
    } else {
        chose.push(
            match if abc[last.0] < abc[last.1] {
                last.0
            } else {
                last.1
            } {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                _ => unreachable!(),
            },
        )
    }

    println!("{}", ans);
    if ans == "Yes" {
        println!("{}", chose.iter().join("\n"));
    }
}

fn to_abc<'a>(abc: &'a mut [usize], now: &String) -> (&'a mut usize, &'a mut usize) {
    let ptr = abc.as_mut_ptr();
    match &now[..] {
        "AB" => unsafe { (&mut *ptr, &mut *ptr.add(1)) },
        "BC" => unsafe { (&mut *ptr.add(1), &mut *ptr.add(2)) },
        "AC" => unsafe { (&mut *ptr, &mut *ptr.add(2)) },
        _ => unreachable!(),
    }
}

fn common_abc(now: &String, next: &String, abc: &Vec<usize>) -> Option<char> {
    match (&now[..], &next[..]) {
        ("AB", "BC") | ("BC", "AB") => Some('B'),
        ("BC", "AC") | ("AC", "BC") => Some('C'),
        ("AC", "AB") | ("AB", "AC") => Some('A'),
        (s, t) if s == t => Some({
            match s {
                "AB" => {
                    if abc[0] >= abc[1] {
                        'A'
                    } else {
                        'B'
                    }
                }
                "BC" => {
                    if abc[1] >= abc[2] {
                        'B'
                    } else {
                        'C'
                    }
                }
                "AC" => {
                    if abc[0] >= abc[2] {
                        'A'
                    } else {
                        'C'
                    }
                }
                _ => unreachable!(),
            }
        }),
        _ => None,
    }
}
