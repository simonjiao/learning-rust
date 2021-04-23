use error_chain::error_chain;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

error_chain! {}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

pub fn global_mutable_state() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("pear")?;
    let handle = thread::spawn(
        || match FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard") {
            Ok(db) => db
                .iter()
                .enumerate()
                .for_each(|(i, item)| println!("{} {}", i, item)),
            Err(err) => println!("{}", err),
        },
    );
    insert("grape")?;
    handle.join().unwrap();
    Ok(())
}
