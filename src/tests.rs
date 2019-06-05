use crate::Toc;

const TOC: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/gii-toc.xml"));

#[test]
fn test_load_toc() {
    let toc = Toc::from_str(TOC).expect("can't parse toc");

    assert_eq!(toc.items.len(), 6419);

    assert_eq!(toc.items[0].title, "Gesetz über die Ausprägung einer 1-DM-Goldmünze und die Errichtung der Stiftung \"Geld und Währung\"");
    assert_eq!(toc.items[0].link, "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip");
}
