fn main() {
    println!("cargo:rustc-link-arg=/DEF:./winmm.def");
}
