//! The problems in content/ are embedded with `include_dir!`, which can't
//! tell cargo on stable Rust that they're inputs: this does, so an edit to a
//! problem rebuilds the binary.

fn main() {
    println!("cargo:rerun-if-changed=content");
}
