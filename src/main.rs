mod class;

fn main() {
    let g = class::Class::new("Test", 1);

    println!("{:#?}", g);
}
