extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/tests.cpp")
        .include(".")
        .include("../assimp/include")
        .compile("tests");
}
