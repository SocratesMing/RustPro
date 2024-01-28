mod test_demo;
mod trait_demo;
mod unsafe_demo;

pub(crate) fn main() {
    unsafe_demo::main();
    trait_demo::main();
}
