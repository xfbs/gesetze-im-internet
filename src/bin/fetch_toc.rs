extern crate lawapi_de;

fn main() {
    let toc = lawapi_de::gesetz::Toc::fetch().unwrap();

    for item in toc.items {
        println!("{:?}", item);
    }
}
