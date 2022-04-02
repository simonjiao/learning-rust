use crossbeam_channel::unbounded;
use std::thread;
use std::time::Duration;

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
