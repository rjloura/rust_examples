use crossbeam_deque::{Injector, Steal};
use std::sync::Arc;
use std::thread;
use threadpool::ThreadPool;
use uuid::Uuid;

#[derive(Debug)]
enum QueueMsg {
    Work(QueueWork),
    Quit,
}

#[derive(Debug)]
struct QueueWork {
    id: Uuid,
    number: u32,
}

fn main() {
    let queue = Arc::new(Injector::<QueueWork>::new());
    let (tx, rx) = crossbeam_channel::bounded(10);

    let gen_queue = Arc::clone(&queue);
    let gen = thread::spawn(move || {
        for i in 0..100 {
            let work = QueueMsg::Work(QueueWork {
                id: Uuid::new_v4(),
                number: i,
            });

            tx.send(work).expect("sending");
        }

        // Send the quit message
        tx.send(QueueMsg::Quit).expect("sending quit");
    });

    let broker = thread::spawn(move || {
        let mut pool = ThreadPool::new(10);
        loop {
            let msg = match rx.recv() {
                Ok(m) => m,
                Err(_) => break,
            };

            let work = match msg {
                QueueMsg::Work(w) => w,
                QueueMsg::Quit => break,
            };

            if work.number == 50 {
                pool.set_num_threads(30);
            } else if work.number == 60 {
                pool.set_num_threads(1);
            }

            gen_queue.push(work);

            // Don't queue up a new worker if we are already at the max.
            if pool.active_count() + pool.queued_count() >= pool.max_count() {
                continue;
            }

            let w_queue = Arc::clone(&queue);

            pool.execute(move || {
                let mut cont = true;

                while cont {
                    let s = w_queue.steal();
                    match s {
                        Steal::Success(_) => {
                            thread::sleep(std::time::Duration::from_millis (500));
                            cont = true;
                            continue;
                        }
                        Steal::Empty => {
                            println!("Queue empty");
                            cont = false;
                        }
                        Steal::Retry => {
                            println!("Queue retry");
                            cont = false;
                        }
                    }
                }
            });

            println!("=================");
            // Queued Count is the number of threads that have been created
            // but are not running on the pool.  i.e. The number of "threads"
            // (functions) that are waiting for an available thread to run on.
            println!("Queued Count: {}", pool.queued_count());

            // Active Count is the number of threads currently running on the
            // pool.
            println!("Active Count: {}", pool.active_count());

            // Max Count is the total number of threads in the pool.
            println!("Max Count: {}", pool.max_count());
        }
        pool.join();
    });

    gen.join().expect("gen thread");
    broker.join().expect("broker");
}
