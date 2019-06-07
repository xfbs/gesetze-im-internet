use crate::Toc;

const TOC: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/gii-toc.xml"));

#[test]
fn test_load_toc() {
    let toc = Toc::from_str(TOC).expect("can't parse toc");

    assert_eq!(toc.items.len(), 6419);

    assert_eq!(toc.items[0].title, "Gesetz über die Ausprägung einer 1-DM-Goldmünze und die Errichtung der Stiftung \"Geld und Währung\"");
    assert_eq!(
        toc.items[0].link,
        "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip"
    );
}

mod toc_item {
    use crate::TocItem;

    #[test]
    fn test_can_create_toc_item() {
        let gesetz = TocItem::new("Abgeordnetengesetz".into(), "ABG".into());
        assert_eq!(gesetz.title, "Abgeordnetengesetz");
        assert_eq!(gesetz.link, "ABG");
    }

    #[test]
    fn test_can_compare_toc_item() {
        let gesetz_a = TocItem::new("A".into(), "A".into());
        let gesetz_b = TocItem::new("B".into(), "A".into());
        let gesetz_c = TocItem::new("A".into(), "C".into());

        assert_ne!(gesetz_a, gesetz_b);
        assert_ne!(gesetz_a, gesetz_c);
        assert_ne!(gesetz_b, gesetz_c);

        assert_eq!(gesetz_a, gesetz_a);
        assert_eq!(gesetz_b, gesetz_b);
        assert_eq!(gesetz_c, gesetz_c);
    }

    #[test]
    fn test_can_parse_short() {
        let item = TocItem::new(
            "Gesetz".into(),
            "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip".into(),
        );
        assert_eq!(item.short(), Some("1-dm-goldm_nzg"));
    }

    #[test]
    fn test_can_parse_url() {
        let item = TocItem::new(
            "Gesetz".into(),
            "http://www.gesetze-im-internet.de/1-dm-goldm_nzg/xml.zip".into(),
        );
        let url = item.url().unwrap();
        assert_eq!(url.host_str(), Some("www.gesetze-im-internet.de"));
    }
}
