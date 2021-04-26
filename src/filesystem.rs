use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
    }
}

mod read_file;
pub use read_file::read_lines;

mod my_same_file;
pub use my_same_file::same_file_rw;

mod memmap_acc;
pub use memmap_acc::memmap_acc;

mod traverse;
pub use traverse::*;
