
fn main() {
    //use learning_rust::cal_sha256;
    //if let Err(e) = cal_sha256() {
    //    println!("Error: {}", e);
    //}
    //use learning_rust::verify_hmac;
    //if let Err(e) = verify_hmac() {
    //    println!("Error: {}", e);
    //}
    use learning_rust::salt_hash_passwd;
    if let Err(e) = salt_hash_passwd() {
        println!("Error: {}", e);
    }
}