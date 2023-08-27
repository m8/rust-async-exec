use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};

use std::{
    future::Future,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    sync::{Arc, Mutex},
    time::Duration,
    task::{Context, Poll, Waker},
};

// The timer we wrote in the previous section:
use timer_future::TimerFuture;

#[derive(Debug)]
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {

        println!("wake_by_ref called");
        
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

impl Executor {

    pub fn new() -> Executor {
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        Executor { ready_queue, task_sender }
    }
    
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }

    pub fn run(&self) {

        while let Ok(task) = self.ready_queue.recv() {
    
            println!("received a task");
            
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
    
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);

                    println!("Task is not finished yet");
                }
                else {
                    println!("Task is finished");
                }
            }
        }
    }
}







