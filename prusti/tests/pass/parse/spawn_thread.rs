// compile-flags: -Zprint-desugared-specs -Zprint-typeckd-specs -Zskip-verify -Zhide-uuids
// normalize-stdout-test: "[a-z0-9]{32}" -> "$(NUM_UUID)"
// normalize-stdout-test: "[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}" -> "$(UUID)"

#![feature(register_tool)]
#![register_tool(prusti)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use prusti_contracts::*;
use std::thread;

#[ensures="on_join(*x == (old(*x) + 1) * 2)"]
fn spawn_double_thread (mut x : Box<i32>) -> thread::JoinHandle<Box<i32>> {
    *x = *x + 1;
    #[ensures="*x == old(*x) * 2"]
        thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        *x += *x;
        x
    })
}

#[ensures="on_join[0](*x == (old(*x) + 1) * 2)"]
fn spawn_multiple_thread (mut x : Box<i32>) -> Vec<thread::JoinHandle<Box<i32>>> {
    *x = *x + 1;
    #[ensures="*x == old(*x) * 2"]
        let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        *x += *x;
        x
    });
    vec!(t)
}

fn main() {}
