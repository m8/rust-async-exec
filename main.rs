#![allow(dead_code, unused_imports)]

use futures::Future;
use std::{
    task::{Context, Poll, Waker},
    pin::Pin,
};

mod executor;

struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

async fn say_hello() {
    println!("hello");
    YieldNow { yielded: false }.await;
    println!("world");
}

fn main() {

    let exec = executor::Executor::new();

    exec.spawn(say_hello());

    exec.spawn(async {
        println!("hey!");
    });

    exec.run();
}

