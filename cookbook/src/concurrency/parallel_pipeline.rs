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
