use learning_rust::sqlite_db_ops;

fn main() {
   if let Err(e) = sqlite_db_ops() {
       println!("Error: {}", e);
   }
}