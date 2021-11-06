use std::env;

fn main() 
{
let out_dir = env::var_os("OUT_DIR").unwrap();
println!("OUT_DIR = {:?}", out_dir);
println!("cargo:rerun-if-changed=build.rs");
}
