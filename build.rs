// Build script to handle custom output directory
// Copies the compiled binary from target/release to dist/

fn main() {
    // Tell cargo to only run this script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // The actual binary copying will be handled by a post-build step
    // via cargo's output directory mechanism
}
