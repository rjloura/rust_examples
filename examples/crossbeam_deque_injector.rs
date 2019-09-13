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

            let w_queue = Arc::clone(&queue);
            pool.execute(move || {
                let mut cont = true;

                while cont {
                    let s = w_queue.steal();
                    match s {
                        Steal::Success(_) => {
                            thread::sleep(std::time::Duration::from_millis(500));
                            cont = true;
                            continue;
                        }
                        Steal::Empty => {
                            println!("Queue was empty");
                            cont = false;
                        }
                        Steal::Retry => {
                            println!("Queue was retry");
                            cont = false;
                        }
                    }
                }
            });
            println!("Active Count: {}", pool.active_count());
            println!("Max Count: {}", pool.max_count());
            println!("Queued Count: {}", pool.queued_count());
        }
        pool.join();
    });

    gen.join().expect("gen thread");
    broker.join().expect("broker");
}
