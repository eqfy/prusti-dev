# Supporting Concurrency Via Threads in Prusti

## Introduction
As a programming language focusing on safety and efficiency, safe concurrency is a distinguishing difference between Rust and older languages like C. Safe concurrency is also heavily used in many applications to boost efficiency. 

Despite Rust's advanced features for safe concurrency, Rust is unable to prove program correctness. As an automatic program verifier based on the Viper infrastructure, Prusti allows Rust programmer to easily specify their programs for automatic formal verfication. 

This document discusses about the design and implementation for extending Prusti with capabilities to verify concurrent programs using threads. 

The document will be split over 4 sections:

1. Specification Syntax Design
2. Viper Encoding Design
3. Parsing and Typechecking Implementation
5. MIR to Viper Implementation

## A User's Perspective

This project will currently only support threads instantiated with closures created in the spawn statement. 

#### Unsupported usages:
+ Instantiating threads with a closure when the closure is not defined in the spawn
+ Using old expressions in `#[ensures(...)]` when returning joinHandles of threads
+ 

## Specification Syntax

### Thread spawn and join in the same function

In general, the specification of threads is a straightforward specification of the post-condition (i.e. what happens after the thread joins). Normally, a thread is spawned by passing in a zero-argument closure as the argument. In our case, the specifications are appended before this closure. See ([Fig 1](####Fig-1)) for a general case.

#### Fig 1
```rust=
#![feature(register_tool)]
#![register_tool(prusti)]
// The following compiler flags are necessary for proc_macros 
// to work at expression positions with custom attributes
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

// The necessary imports
extern crate prusti_contracts;
use prusti_contracts::*;
use std::thread; // can be inlined as well
...
fn test() {
    // Spawning the thread
    // Note we can still provide a postcondition without the let statement
    let t = thread::spawn(
        #[t_ensures(...)]
        || {
            // The thread body
            ...
        }
    );
    ...
    // Joining the thread
    t.join().unwrap();
}
...
```

It was decided to not allow the user to specify pre-conditions because of threads are spawned (forked) in Rust. Rust threads are created generally with the the `thread::spawn(|| {})` syntax, where the closure is in an inlined position. This makes specifications for pre-conditions mostly redundant. Alternatively, the user can also pass in a closure, but this is rarely used in practice.

The following ([Fig 2](####Fig-2)) is a basic example where the `#[t_ensures(...)]` before the closure (line 9) makes sure that the `x` captured will be one more than its original by the time the thread is joined. The `old(x)` used in the specifications refers to the state of x when it is captured by the closure. 

Note that `old()` is not supported in some cases (see [unsupported usages](####Unsupported-usages)).

#### Fig 2
```rust=
...
#[requires(true)]
#[ensures(result == 2)]
fn add1() -> i32 {
    let x: i32 = 1;
    let t: JoinHandle<i32> = thread::spawn(
        // old() refers to the state of x when it's captured by the closure
        #[t_ensures(result == old(x) + 1)]
        move || { 
            x = x + 1
    });
    let res: i32 = t.join().unwrap();
    res
}
...
```

### Thread spawn and join in separate functions
Sometimes, the user may want to have the spawn a thread in a separate function and return the `JoinHandle<T>` to the thread spawned. In these cases, we allow users to include a thread's postcondition into the postcondition of the Rust function with the `on_join()` syntax.

Currently, this only specification only supports one state assertions so `old()` statements cannot be used. The reason behind this is because the Viper backend we use to verify our program does not support passing labels of states yet.

#### Fig 3a
```rust=
...
#[requires(x > 0)]
#[ensures(on_join(result > 0))]
fn spawn_add1(x : i32) -> JoinHandle<i32> {
    // What if there is x is changed here?
    let t: JoinHandle<i32> = thread::spawn(
        #[t_ensures(result > 0)]
        move || {
            x = x + 1
    });
    ...
    t
}
...
```
In the above example (Fig 3a), the value of x might be changed before the spawning of a thread. In simple cases, we can tweek our function's post-condition (Fig 3b). Though this example is not technically supported right now because it uses `old` statements.

#### Fig 3b
```rust=
...
#[requires(x > 0)]
#[ensures(on_join(result == old(x + 1) * 2))]
fn spawn_add1times2(x : i32) -> JoinHandle<i32> {
    x = x + 1; // x is changed here
    let t: JoinHandle<i32> = thread::spawn(
        #[t_ensures(result == old(x) * 2)]
        move || {
            x = x + 1
    });
    ...
    t
}
...
```

In the future, Prusti is also likely to support a function returning a some collection of `JoinHandles<T>`. The envisioned syntax will look like `on_join[n]` (n is some integer) with the `[ ]` operator refering to the postcondition of the thread in the order that they are added. (Fig 3c)

Fig 3c
```rust=
#[requires(x > 0)]
#[ensures(on_join[0](result > 0) && on_join[1](...) && ...)]
fn spawn_add1(x : i32) -> Vec<JoinHandle<i32>> {
    // What if there is x is changed here?
    let t1: JoinHandle<i32> = thread::spawn(
        #[t_ensures(result > 0)]
        move || {
            x = x + 1
    });
    let t1: JoinHandle<i32> = thread::spawn(
        ...
    });
    ...
    vec![t1, t2]
}
...
```

## Viper Encoding
We use an inline approach to encoding threads in Viper. Specifically, for threads are 
In Fig 4, we have an example of how Fig 2 will be encoded in Viper if we did it by hand. Technically, many features like requires and ensures as well as naming of 
#### Fig 4

```
predicate join_handle(self: Ref)

method simple_thread(x: Ref) returns (res: Ref)
    requires i32(x)
    requires unfolding i32(x) in (x.val_int) > 0
    ensures i32(res)
    ensures unfolding i32(res) in (res.val_int) == old(unfolding i32(x) in (x.val_int) * 2)
{
    // Inline thread and verify
    label l0;
    var t0: Ref
    var b : Bool
    b := havoc_bool()
    if (b) {
        exhale forall i:Ref :: perm(i32(i)) > none && i != x  ==> acc(i32(i), perm(i32(i)))
        unfold i32(x)
        x.val_int := x.val_int * 2
        fold i32(x)
        exhale i32(x) && unfolding i32(x) in (x.val_int) == old[l0](unfolding i32(x) in (x.val_int) * 2)
        assume false
    } else {
        exhale i32(x)
        inhale join_handle(t0) --* i32(x) && unfolding i32(x) in (x.val_int) == old[l0](unfolding i32(x) in (x.val_int) * 2)
    }
    // Joining a thread
    inhale join_handle(t0)
    apply join_handle(t0) --* i32(x) && unfolding i32(x) in (x.val_int) == old[l0](unfolding i32(x) in (x.val_int) * 2)
    inhale i32 (res)
    unfold i32(res)
    unfold i32(x)
    res.val_int := x.val_int
    fold i32(x)
    fold i32(res)
}
```

Technically, the requires and ensures statements for non-pure Rust functions are replaced by asserts, inhales and exhales. The above code is 
## Parsing and Typechecking
This section covers the implementation details for parsing and typchecking the specifications.
### The choice of macros

We will be using the 

### Inputs and Outputs

fj




