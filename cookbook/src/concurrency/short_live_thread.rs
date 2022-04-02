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
