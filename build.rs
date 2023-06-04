use std::env;

fn main() {
    let _out_dir = env::var_os("OUT_DIR").unwrap();
    match pkg_config::find_library("libarchive") {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
    }
}
