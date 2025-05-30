// Created by Ayush Biswas at 2025/05/20 09:07
// https://codeforces.com/problemset/problem/1984/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::{collections::HashSet, io::Read};

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let last_nums = a.get(1..n).unwrap().iter().collect::<HashSet<_>>();
    if last_nums.len() != 1 {
        "YES\n".to_string() + "R" + &vec!["B"; n - 1].into_iter().collect::<String>()
    } else if a[0] != a[1] {
        "YES\n".to_string() + "BR" + &vec!["B"; n - 2].into_iter().collect::<String>()
    } else {
        "NO".into()
    }
}
// @code end
