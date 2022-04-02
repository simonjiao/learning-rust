use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc;
use threadpool::ThreadPool;
use walkdir::WalkDir;

fn is_type(entry: &Path, ext: &str) -> bool {
    matches!(entry.extension(), Some(e) if e.to_string_lossy().to_lowercase() == ext)
}

//fn compute_digest(path: &str) -> String {
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buf = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buf)?;
        if count == 0 {
            break;
        }
        context.update(&buf[..count]);
    }

    Ok((context.finish(), filepath))
}

pub fn calc_sha256sum() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = mpsc::channel();

    for entry in WalkDir::new("/sharedfs/myspace/se.src.bak")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_type(e.path(), "tar"))
    {
        let path = entry.path().to_owned();
        let digest = compute_digest(path);
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(digest).expect("Could sent data");
        });
    }
    // Close the channel, so we can exit
    drop(tx);

    for t in rx.iter() {
        let (digest, path) = t?;
        println!("{:?} {:?}", digest, path);
    }

    Ok(())
}
