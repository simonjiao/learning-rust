use learning_rust::execute_query;

fn main(){
    env_logger::init();

    execute_query("DROP TABLE students");
}