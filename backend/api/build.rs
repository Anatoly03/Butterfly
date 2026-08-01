//! The build script of the Butterfly backend application.

fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
}
