extern crate gcc;

fn main() {
    gcc::compile_library("libpredicates.a", &["src/predicates.c"]);
}
