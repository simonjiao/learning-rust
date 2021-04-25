//use learning_rust::calc_sha256sum;
//use learning_rust::draw_fractal;
//use learning_rust::find_max;
//use learning_rust::global_mutable_state;
//use learning_rust::mutate_array;
//use learning_rust::parallel_pipeline;
//use learning_rust::send_data_between_threads;
//use learning_rust::par_test_array;
//use learning_rust::par_search_array;
//use learning_rust::par_sort_array;
//use learning_rust::map_reduce;
use learning_rust::jpg_thumbnails;

fn main() {
    //mutate_array();
    //par_test_array();
    //par_search_array();
    //par_sort_array();
    //map_reduce();
    if let Err(e) = jpg_thumbnails() {
        println!("Failed to save thumbnails {}", e);
    }

    //let arr = &[1, 22, -4, 10];
    //let max = find_max(arr);
    //assert_eq!(max, Some(22));

    //parallel_pipeline();

    //send_data_between_threads();
    //global_mutable_state().unwrap();

    //if let Err(err) = calc_sha256sum() {
    //    println!("Failed to calculate sha256sum: {}", err);
    //}

    //if let Err(err) = draw_fractal() {
    //    println!("Failed to draw fractal {}", err);
    //}
}
