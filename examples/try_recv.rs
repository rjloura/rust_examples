use crossbeam_channel;
use crossbeam_channel::TryRecvError;
use std::thread;

struct ControlMsg {}

fn main() {
    let (tx, rx) = crossbeam_channel::bounded(1);

    // Two examples here:

    // 1. Sending via alternate thread
    let gen = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(1));
        tx.send(ControlMsg {}).expect("sending");
    });

    // 2. Sending all at once then dropping the tx before calling try_recv()
    // Even after the tx is dropped the try_recv can still receive the
    // ControlMsg sent.
    /*
    tx.send(ControlMsg {}).expect("sending");
    drop(tx);
    */

    thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(_) => {
                println!("Msg Received");
                break;
            }
            Err(e) => match e {
                TryRecvError::Empty => {
                    println!("Continuing");
                    continue;
                }
                TryRecvError::Disconnected => {
                    println!("Disconnected returning");
                    break;
                }
            },
        }
    })
    .join()
    .expect("Join try_recv");

    gen.join().expect("Join gen");
}
