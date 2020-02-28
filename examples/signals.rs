use signal_hook::{self, iterator::Signals};
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Default)]
struct Config {
    runnable: Mutex<bool>,
}

fn signal_handler(tx: crossbeam_channel::Sender<()>) -> impl Fn() {
    move || {
        let signals = Signals::new(&[signal_hook::SIGTERM])
            .expect("register signals");
        for signal in signals.forever() {
            match signal {
                signal_hook::SIGTERM => {
                    println!("signal received");
                    tx.send(()).expect("send");
                }
                _ => unreachable!(),
            }
        }
    }
}


// The updater receives a message from the signal handler to do some work
// that may not be safe or wise inside the context of a signal handler.
fn updater(
    rx: crossbeam_channel::Receiver<()>,
    config: Arc<Config>,
) -> JoinHandle<()> {
    thread::spawn(move || loop {
        match rx.recv() {
            Ok(()) => {
                let mut runnable = config.runnable.lock().expect("lock");
                *runnable = true;
            }
            Err(_) => {
                return;
            }
        }
    })
}

// This function simply simulates the portion of the rest of your program
// that is interested in whatever value needs to be updated when a signal is
// received.
fn periodic_checker(config: Arc<Config>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3));
            dbg!(&config.runnable);

            // Set it back to false
            let mut runnable = config.runnable.lock().expect("lock");
            *runnable = false;
        }
    })
}

fn main() -> Result<(), Error> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    let config = Config {
        runnable: Mutex::new(false),
    };
    let conf = Arc::new(config);

    let update_handle = updater(rx, Arc::clone(&conf));
    let checker_handle = periodic_checker(Arc::clone(&conf));

    println!("Send SIGTERM to {}", std::process::id());

    thread::spawn(signal_handler(tx)).join().expect("join handler");
    update_handle.join().expect("join update");
    checker_handle.join().expect("join checker");

    Ok(())
}
