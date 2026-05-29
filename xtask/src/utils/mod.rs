pub mod file;
pub mod paths;

pub use file::{
    create_range_vec, generate_file, generate_keyed_lookup, intern_table, save_bin_compressed,
};
pub use paths::root;
