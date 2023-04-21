fn main() {
    cc::Build::new().file("./link/test.c").compile("test.a");
}
