fn main() {
    uniffi::generate_scaffolding("./include/paper.udl").expect("Scaffolding uniffi must work");
}
