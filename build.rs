extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/predicates.c")
		.opt_level(2)
                .compile("libpredicates.a");
}
