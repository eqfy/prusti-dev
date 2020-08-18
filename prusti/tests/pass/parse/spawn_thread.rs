// compile-flags: -Zprint-desugared-specs -Zprint-typeckd-specs -Zskip-verify -Zhide-uuids
// normalize-stdout-test: "[a-z0-9]{32}" -> "$(NUM_UUID)"
// normalize-stdout-test: "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}" -> "$(UUID)"

#![feature(register_tool)]
#![register_tool(prusti)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use prusti_contracts::*;
use std::thread;

#[ensures(on_join(result, result == 1))]
fn spawn_thread () -> thread::JoinHandle<i32> {
    thread::spawn(
        #[t_ensures(result == 1)] || -> i32 {
        1
    })
}

#[requires(on_join(t, result == 1))]
#[ensures(result == 2)]
fn receive_thread (t : thread::JoinHandle<i32>) -> i32 {
    t.join().unwrap() + 1
}

// #[ensures(on_join(result, *result == (old(*x) + 1) * 2))]
// fn spawn_double_thread (mut x : Box<i32>) -> thread::JoinHandle<Box<i32>> {
//     *x = *x + 1;
//     thread::spawn(
//         #[t_ensures(*x == old(*x) * 2)] move || -> Box<i32> {
//         thread::sleep(Duration::from_millis(50));
//         *x += *x;
//         x
//     })
// }
// Unsupported
// #[ensures="on_join(result[0], *result == (old(*x) + 1) * 2)"]
// fn spawn_multiple_thread (mut x : Box<i32>) -> Vec<thread::JoinHandle<Box<i32>>> {
//     *x = *x + 1;
//     #[ensures="*x == old(*x) * 2"]
//         let t = thread::spawn(move || -> Box<i32> {
//         thread::sleep(Duration::from_millis(50));
//         *x += *x;
//         x
//     });
//     vec!(t)
// }

fn main() {}
