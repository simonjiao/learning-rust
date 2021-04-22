mod global_mutable_state;
pub use global_mutable_state::global_mutable_state;
mod sha256sum;
pub use sha256sum::calc_sha256sum;

// spawn a short lived thread
pub fn find_max(arr: &'static [i32]) -> Option<i32> {
    const THREADSHOLD: usize = 2;

    if arr.len() <= THREADSHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    //use std::thread;
    //let handle = thread::spawn(move || find_max(left));
    //let max = handle.join().unwrap();

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;
// create a parallel pipeline
pub fn parallel_pipeline() {
    let (tx1, rx1) = bounded(1);
    let (tx2, rx2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // one producer thread
        s.spawn(|_| {
            for i in 0..n_msgs {
                tx1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // Close the channel
            drop(tx1);
        });

        for _ in 0..n_workers {
            let (tx, rx) = (tx2.clone(), rx1.clone());
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in rx.iter() {
                    println!("worker {:?} received {}", thread::current().id(), msg);
                    tx.send(msg * 2).unwrap();
                }
            });
        }

        // Close the channel
        drop(tx2);

        for msg in rx2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}

use crossbeam_channel::unbounded;

pub fn send_data_between_threads() {
    let (tx, rx) = unbounded();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    for _ in 0..n_msgs {
        let msg = rx.recv().unwrap();
        println!("Received {:?}", msg);
    }
}
