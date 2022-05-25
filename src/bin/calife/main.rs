mod config;

#[cfg(not(unix))]
compile_error!("Not available outside of the Unix World");

use calife_rs::version::{CALIFE_NAME, CALIFE_VERSION};

fn main() {
    println!("Hello, world! {} = {}", CALIFE_NAME,  CALIFE_VERSION);
}
