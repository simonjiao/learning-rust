//use learning_rust::read_lines;
//use learning_rust::same_file_rw;
//use learning_rust::memmap_acc;
//use learning_rust::find_recent_modified_file;
//use learning_rust::contains_loop;
//use learning_rust::find_dup_filename;
//use std::path::PathBuf;
//use learning_rust::find_by_ext;
//use learning_rust::skip_dotfiles;
//use learning_rust::cal_size;

fn main() {
    //if let Err(e) = read_lines() {
    //    println!("Failed to read lines from file {}", e);
    //}

    //if let Err(e) = same_file_rw() {
    //    println!("Failed: {}", e);
    //}

    //if let Err(e) = memmap_acc() {
    //    println!("Failed : {}", e);
    //}

    //if let Err(e) = find_recent_modified_file(24 * 3600) {
    //    println!("{}", e);
    //}
    //assert_eq!(
    //    contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
    //    Some((
    //        PathBuf::from("/tmp/foo"),
    //        PathBuf::from("/tmp/foo/bar/baz/qux")
    //    ))
    //);

    //find_dup_filename();
    //find_by_ext(".json").unwrap();
    //skip_dotfiles();
    //cal_size(1, 3);

    //use learning_rust::find_json;
    //if let Err(e) = find_json() {
    //    println!("{}", e);
    //}

    use learning_rust::glob_with_options;
    if let Err(e) = glob_with_options() {
        println!("{}", e);
    }
}
