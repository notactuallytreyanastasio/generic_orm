fn main() {
    orm::init(None).unwrap().run_all_blocking();
}