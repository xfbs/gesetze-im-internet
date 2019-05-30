extern crate lawapi_de;

fn main() {
    let toc = lawapi_de::gesetz::Toc::fetch().unwrap();

    toc.items[10].fetch().unwrap();
}
