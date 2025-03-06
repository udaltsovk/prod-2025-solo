fn main() {
    println!("cargo:rerun-if-changed=db");

    #[cfg(test)]
    std::env::set_var("RUST_LOG", "debug");
}
