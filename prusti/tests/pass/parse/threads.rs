// compile-flags: -Zprint-desugared-specs -Zprint-typeckd-specs -Zskip-verify -Zhide-uuids
// normalize-stdout-test: "[a-z0-9]{32}" -> "$(NUM_UUID)"
// normalize-stdout-test: "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}" -> "$(UUID)"

#![feature(register_tool)]
#![register_tool(prusti)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use prusti_contracts::*;
use std::thread;

// fn test1 () {
//     thread::spawn(#[t_ensures(true)] || -> () {});
// }


#[ensures(result == old(x))]
fn test2 (x : i32) -> i32 {
    let t = thread::spawn(
        #[t_ensures(result == old(x))] move || -> i32 {
        x
    });
    t.join().unwrap()
}

// #[requires(x > 0)]
// #[ensures(result > 0 && result == x * 2)]
// fn test3 (x : i32) -> i32 {
//     let t = thread::spawn(
//         #[t_ensures(result > 0 && result == x * 2)] move || -> i32 {
//         x * 2
//     });
//     t.join().unwrap()
// }
//
// #[requires(*x > 0)]
// #[ensures(result > 0 && result == old(*x) * 2)]
// fn test4 (x : &'static mut i32) -> i32 {
//     let t = thread::spawn(
//         #[t_ensures(*result > 0 && *result == old(*x) * 2)] move || -> &mut i32 {
//             *x = *x * 2;
//             x
//     });
//     let res = t.join().unwrap();
//     *res
// }
//
// #[ensures(result == old(*x) * 2 + old(*y) * 3)]
// fn test5 (mut x : Box<i32>, mut y : Box<i32>) -> i32 {
//     let t1 = thread::spawn(
//         #[t_ensures(*result == old(*x) * 2)] move || -> Box<i32> {
//         *x += *x;
//         x
//     });
//     let t2 = thread::spawn(
//         #[t_ensures(*result == old(*x) * 3)] move || -> Box<i32> {
//         *y = *y * 3;
//         y
//     });
//     x = t1.join().unwrap();
//     y = t2.join().unwrap();
//     *x + *y
// }

fn main() {}
