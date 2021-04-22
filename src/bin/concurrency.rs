//use learning_rust::global_mutable_state;
//use learning_rust::find_max;
//use learning_rust::parallel_pipeline;
//use learning_rust::send_data_between_threads;
use learning_rust::calc_sha256sum;

fn main() {
    //let arr = &[1, 22, -4, 10];
    //let max = find_max(arr);
    //assert_eq!(max, Some(22));

    //parallel_pipeline();

    //send_data_between_threads();
    //global_mutable_state().unwrap();

    if let Err(err) = calc_sha256sum() {
        println!("Failed to calculate sha256sum: {}", err);
    }
}
