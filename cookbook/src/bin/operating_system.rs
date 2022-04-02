fn  main() {
    use learning_rust::run_git_log;
    if let Err(e) = run_git_log() {
        eprintln!("{}", e);
    }
}