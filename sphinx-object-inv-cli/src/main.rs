// cat samples/objects_python.inv | cargo run
fn main() {
    println!(
        "{:?}",
        sphinx_object_inv::SphinxObjectInv::from(&mut std::io::BufReader::new(std::io::stdin()))
            .unwrap()
    );
}
