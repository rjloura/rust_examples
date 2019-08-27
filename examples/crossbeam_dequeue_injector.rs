// This is a dirty example of how use crossbeam dequeue.  It is dirty to
// reduce the clutter associated with things like channels and async/await. You
// certainly do not want to use this design in production, it is only here to
// show how an injector queue works.

// A better example would be to include crossbeam channels and have a
// single thread that "steals" the next QueueWork object and spawns a worker
// thread to do work on it.  Then a final message is sent via this channel
// to the parent worker thread to tell it to shutdown.  This could leverage
// something like select() or just a simple enum with two variants (e.g. work,
// control_msg)
// This would also be a great use case for async/await.
// Currently we exit on queue empty which again is a horrible "design"
// because the queue could run dry while the generator thread is still
// running and simply hasn't put another object into the queue yet.

use crossbeam_deque::{Injector, Steal};
use std::sync::Arc;
use std::thread;
use uuid::Uuid;

#[derive(Debug)]
struct QueueWork {
    id: Uuid,
    name: String,
}

fn main() {
    let queue = Arc::new(Injector::<QueueWork>::new());
    let mut workers = vec![];

    let gen_queue = Arc::clone(&queue);
    let gen = thread::spawn(move || {
        for i in 0..10 {
            let work = QueueWork {
                id: Uuid::new_v4(),
                name: i.to_string(),
            };

            gen_queue.push(work);
        }
    });

    while queue.is_empty() {}

    for _ in 0..2 {
        let w_queue = Arc::clone(&queue);
        workers.push(thread::spawn(move || {
            let mut cont = true;

            while cont {
                let s = w_queue.steal();
                if let Steal::Success(w) = s {
                    println!("Thread: {:#?} | data: {:#?}", thread::current(), w);

                    cont = true;
                    continue;
                }

                if s.is_empty() {
                    println!("Queue was empty");
                } else if s.is_retry() {
                    println!("Queue was retry");
                } else {
                    panic!("Not empty or retry");
                }
                cont = false;
            }
        }))
    }

    gen.join().expect("gen thread");
    for w in workers {
        w.join().expect("worker thread");
    }
}
