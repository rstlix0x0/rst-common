use rst_common::standard::uuid::Uuid;

fn main() {
    let uid = Uuid::new_v4();
    println!("{}", uid)
}