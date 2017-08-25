extern crate gcc;

fn main() {
    gcc::Build::new()
        .file("src/tests.cpp")
        .include(".")
        .include("../assimp/include")
        .compile("tests");
}
