fn main() {
    //use learning_rust::unpack;
    //if let Err(e) = unpack() {
    //    println!("Error: {}", e);
    //}

    //use learning_rust::pack_dir;
    //if let Err(e) = pack_dir() {
    //    println!("Error: {}", e);
    //}

    use learning_rust::unpack_dir;
    if let Err(e) = unpack_dir() {
        println!("Error: {}", e);
    }
}