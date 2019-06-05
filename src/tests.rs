use crate::Toc;

const TOC: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/gii-toc.xml"));

#[test]
fn test_load_toc() {
    let toc = Toc::from_str(TOC).expect("can't parse toc");

    assert_eq!(toc.items.len(), 6419);
}
