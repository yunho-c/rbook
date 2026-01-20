use crate::epub::util::TestEpub::{
    Epub2NcxNestedDir,
    Epub3DeepNestingDir,
    Epub3EdgeHrefsDir,
    Epub3File,
    Epub3MultiNavDir,
};
use rbook::Ebook;
use rbook::ebook::element::Attributes;
use rbook::ebook::manifest::ManifestEntry;
use rbook::ebook::toc::{Toc, TocChildren, TocEntry, TocEntryKind};
use rbook::epub::metadata::EpubVersion;
use rbook::epub::toc::EpubTocEntry;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn test_toc() {
    let epub = Epub3File.build(|b| b.store_all(true));

    for TocVariantData {
        kind,
        version,
        test_data,
    } in EXPECTED_VARIANTS
    {
        let root = epub.toc().by_kind_version(kind, *version).unwrap();
        let contents = root.children().flatten().collect::<Vec<_>>();

        assert!(root.is_root());
        // The root must contain children
        assert!(!root.children().is_empty());
        assert_eq!(kind, root.kind());
        assert_eq!(test_data.len(), contents.len());

        for (entry, expected) in contents.into_iter().zip(*test_data) {
            assert_eq!(expected.depth, entry.depth());
            assert_eq!(expected.order, entry.order());
            assert_eq!(expected.href, entry.href_raw().unwrap().as_str());
            assert_eq!(expected.label, entry.label());
            assert_eq!(&expected.kind, entry.kind());

            let manifest_entry = entry.manifest_entry().unwrap();
            assert_eq!(entry.href().unwrap().path(), manifest_entry.href());
            // Resources must be identical
            assert_eq!(entry.resource().unwrap(), manifest_entry.resource());
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn test_preference() {
    fn get_test_flag(attributes: Attributes<'_>) -> &str {
        attributes.by_name("rbook:test").unwrap().value()
    }
    let versions = [
        (EpubVersion::EPUB2, "epub2-feature"),
        (EpubVersion::EPUB3, "epub3-feature"),
    ];

    for (version, integrity_check) in versions {
        let epub = Epub3File.build(|b| b.preferred_toc(version).preferred_landmarks(version));
        let toc = epub.toc();
        let toc_root = toc.contents().unwrap();
        let landmarks_root = toc.landmarks().unwrap();

        assert_eq!(
            toc_root,
            toc.by_kind_version(TocEntryKind::Toc, version).unwrap()
        );
        assert_eq!(
            landmarks_root,
            toc.by_kind_version(TocEntryKind::Landmarks, version)
                .unwrap()
        );

        // Check if the provided root is actually the intended one via a flag.
        assert_eq!(integrity_check, get_test_flag(toc_root.attributes()));
        assert_eq!(integrity_check, get_test_flag(landmarks_root.attributes()));
    }
}

#[test]
#[wasm_bindgen_test]
fn test_skip_toc() {
    let epub = Epub3File.build(|b| b.skip_toc(true));
    let toc = epub.toc();

    assert!(toc.contents().is_none());
    assert!(toc.landmarks().is_none());
    assert!(toc.page_list().is_none());
    assert!(toc.kinds().next().is_none());

    for kind in [
        TocEntryKind::Toc,
        TocEntryKind::Landmarks,
        TocEntryKind::PageList,
    ] {
        for version in [EpubVersion::EPUB2, EpubVersion::EPUB3] {
            assert!(toc.by_kind_version(&kind, version).is_none());
        }
        assert!(toc.by_kind(kind).is_none());
    }
}

#[test]
#[wasm_bindgen_test]
fn test_fixture_epub3_multi_nav() {
    let epub = Epub3MultiNavDir.build(|b| b.store_all(true));
    let toc = epub.toc();

    let toc_root = toc
        .by_kind_version(TocEntryKind::Toc, EpubVersion::EPUB3)
        .unwrap();
    let toc_entries = toc_root.children().flatten().collect::<Vec<_>>();
    assert_toc_entries(&toc_entries, EXPECTED_MULTI_NAV_TOC);
    assert!(toc_entries.iter().all(|entry| entry.manifest_entry().is_some()));

    let landmarks_root = toc
        .by_kind_version(TocEntryKind::Landmarks, EpubVersion::EPUB3)
        .unwrap();
    let landmarks_entries = landmarks_root.children().flatten().collect::<Vec<_>>();
    assert_toc_entries(&landmarks_entries, EXPECTED_MULTI_NAV_LANDMARKS);

    let page_list_root = toc
        .by_kind_version(TocEntryKind::PageList, EpubVersion::EPUB3)
        .unwrap();
    let page_list_entries = page_list_root.children().flatten().collect::<Vec<_>>();
    assert_toc_entries(&page_list_entries, EXPECTED_MULTI_NAV_PAGE_LIST);
}

#[test]
#[wasm_bindgen_test]
fn test_fixture_epub3_deep_nesting() {
    let epub = Epub3DeepNestingDir.build(|b| b.store_all(true));
    let toc_root = epub
        .toc()
        .by_kind_version(TocEntryKind::Toc, EpubVersion::EPUB3)
        .unwrap();
    let entries = toc_root.children().flatten().collect::<Vec<_>>();

    assert_eq!(2, toc_root.children().len());
    assert_eq!(5, toc_root.total_len());
    assert_eq!(4, toc_root.max_depth());
    assert_toc_entries(&entries, EXPECTED_DEEP_NESTING_TOC);
}

#[test]
#[wasm_bindgen_test]
fn test_fixture_epub3_edge_hrefs() {
    let epub = Epub3EdgeHrefsDir.build(|b| b.store_all(true));
    let toc_root = epub.toc().contents().unwrap();
    let entries = toc_root.children().flatten().collect::<Vec<_>>();

    assert_toc_entries(&entries, EXPECTED_EDGE_HREFS_TOC);

    for entry in &entries {
        let raw = entry.href_raw().unwrap().as_str();
        let resolved = entry.href().unwrap().as_str();
        assert!(resolved.ends_with(raw));

        if raw == "#local" {
            assert!(entry.manifest_entry().is_none());
        } else {
            assert!(entry.manifest_entry().is_some());
        }
    }
}

#[test]
#[wasm_bindgen_test]
fn test_fixture_epub2_ncx_nested() {
    let epub = Epub2NcxNestedDir.open();
    let toc_root = epub
        .toc()
        .by_kind_version(TocEntryKind::Toc, EpubVersion::EPUB2)
        .unwrap();
    let entries = toc_root.children().flatten().collect::<Vec<_>>();

    assert_toc_entries(&entries, EXPECTED_EPU2_NCX_NESTED_TOC);
}

fn assert_toc_entries(entries: &[EpubTocEntry<'_>], expected: &[TocTestData<'_>]) {
    assert_eq!(expected.len(), entries.len());

    for (entry, expected) in entries.iter().zip(expected) {
        assert_eq!(expected.depth, entry.depth());
        assert_eq!(expected.order, entry.order());
        assert_eq!(expected.href, entry.href_raw().unwrap().as_str());
        assert_eq!(expected.label, entry.label());
        assert_eq!(&expected.kind, entry.kind());
    }
}

/////////////////////////////////////////////////
// TEST DATA
/////////////////////////////////////////////////

pub struct TocVariantData<'a> {
    pub kind: TocEntryKind<'a>,
    pub version: EpubVersion,
    pub test_data: &'a [TocTestData<'a>],
}

impl<'a> TocVariantData<'a> {
    const fn new(
        kind: TocEntryKind<'a>,
        version: EpubVersion,
        test_data: &'a [TocTestData<'a>],
    ) -> Self {
        Self {
            kind,
            version,
            test_data,
        }
    }
}

pub struct TocTestData<'a> {
    pub depth: usize,
    pub order: usize,
    pub href: &'a str,
    pub label: &'a str,
    pub kind: TocEntryKind<'a>,
}

impl<'a> TocTestData<'a> {
    const fn new(
        depth: usize,
        order: usize,
        href: &'a str,
        label: &'a str,
        kind: TocEntryKind<'a>,
    ) -> Self {
        Self {
            depth,
            order,
            href,
            label,
            kind,
        }
    }
}

// Reference: example.epub / example_epub
#[rustfmt::skip]
pub const EXPECTED_VARIANTS: &[TocVariantData] = &[
    TocVariantData::new(TocEntryKind::Toc, EpubVersion::EPUB2, EXPECTED_TOC),
    TocVariantData::new(TocEntryKind::Toc, EpubVersion::EPUB3, EXPECTED_TOC),
    TocVariantData::new(TocEntryKind::Landmarks, EpubVersion::EPUB2, EXPECTED_GUIDE),
    TocVariantData::new(TocEntryKind::Landmarks, EpubVersion::EPUB3, EXPECTED_LANDMARKS),
];
#[rustfmt::skip]
pub const EXPECTED_TOC: &[TocTestData] = &[
    TocTestData::new(1, 1, "EPUB/cover.xhtml", "The Cover", TocEntryKind::Unknown),
    TocTestData::new(1, 2, "EPUB/c1.xhtml?q=1#start", "rbook Chapter 1", TocEntryKind::Unknown),
    TocTestData::new(2, 3, "EPUB/c1a.xhtml", "rbook Chapter 1a", TocEntryKind::Unknown),
    TocTestData::new(1, 4, "EPUB/c2.xhtml", "rbook Chapter 2", TocEntryKind::Unknown),
];
#[rustfmt::skip]
pub const EXPECTED_GUIDE: &[TocTestData] = &[
    TocTestData::new(1, 1, "cover.xhtml", "Cover", TocEntryKind::Cover),
    TocTestData::new(1, 2, "../toc.xhtml", "Table of Contents", TocEntryKind::Toc),
    TocTestData::new(1, 3, "c1.xhtml", "Start Here", TocEntryKind::BodyMatter),
];
#[rustfmt::skip]
pub const EXPECTED_LANDMARKS: &[TocTestData] = &[
    TocTestData::new(1, 1, "EPUB/cover.xhtml", "Cover", TocEntryKind::Cover),
    TocTestData::new(1, 2, "toc.xhtml", "Table of Contents", TocEntryKind::Toc),
    TocTestData::new(1, 3, "EPUB/c1.xhtml", "Start Here", TocEntryKind::BodyMatter),
];

// Reference: fixtures/epub3_multi_nav
#[rustfmt::skip]
pub const EXPECTED_MULTI_NAV_TOC: &[TocTestData] = &[
    TocTestData::new(1, 1, "cover.xhtml", "Cover", TocEntryKind::Unknown),
    TocTestData::new(1, 2, "c1.xhtml", "Chapter 1", TocEntryKind::Unknown),
    TocTestData::new(1, 3, "c2.xhtml", "Chapter 2", TocEntryKind::Unknown),
    TocTestData::new(2, 4, "c2.xhtml#s1", "Section 2.1", TocEntryKind::Unknown),
    TocTestData::new(2, 5, "c2.xhtml#s2", "Section 2.2", TocEntryKind::Unknown),
];
#[rustfmt::skip]
pub const EXPECTED_MULTI_NAV_LANDMARKS: &[TocTestData] = &[
    TocTestData::new(1, 1, "cover.xhtml", "Cover", TocEntryKind::Cover),
    TocTestData::new(1, 2, "c1.xhtml", "Body", TocEntryKind::BodyMatter),
];
#[rustfmt::skip]
pub const EXPECTED_MULTI_NAV_PAGE_LIST: &[TocTestData] = &[
    TocTestData::new(1, 1, "c1.xhtml#p1", "1", TocEntryKind::Unknown),
    TocTestData::new(1, 2, "c2.xhtml#p2", "2", TocEntryKind::Unknown),
];

// Reference: fixtures/epub3_deep_nesting
#[rustfmt::skip]
pub const EXPECTED_DEEP_NESTING_TOC: &[TocTestData] = &[
    TocTestData::new(1, 1, "c1.xhtml#l1", "Level 1", TocEntryKind::Unknown),
    TocTestData::new(2, 2, "c1.xhtml#l1-1", "Level 2", TocEntryKind::Unknown),
    TocTestData::new(3, 3, "c1.xhtml#l1-1-1", "Level 3", TocEntryKind::Unknown),
    TocTestData::new(4, 4, "c1.xhtml#l1-1-1-1", "Level 4", TocEntryKind::Unknown),
    TocTestData::new(1, 5, "c1.xhtml#l2", "Another Root", TocEntryKind::Unknown),
];

// Reference: fixtures/epub3_edge_hrefs
#[rustfmt::skip]
pub const EXPECTED_EDGE_HREFS_TOC: &[TocTestData] = &[
    TocTestData::new(1, 1, "c1.xhtml#p1", "Chapter 1", TocEntryKind::Unknown),
    TocTestData::new(1, 2, "file%20name.xhtml#p2", "File With Spaces", TocEntryKind::Unknown),
    TocTestData::new(1, 3, "#local", "Local Fragment", TocEntryKind::Unknown),
];

// Reference: fixtures/epub2_ncx_nested
#[rustfmt::skip]
pub const EXPECTED_EPU2_NCX_NESTED_TOC: &[TocTestData] = &[
    TocTestData::new(1, 2, "c1.xhtml", "Chapter 1", TocEntryKind::Unknown),
    TocTestData::new(2, 1, "c1.xhtml#s1", "Chapter 1.1", TocEntryKind::Unknown),
    TocTestData::new(1, 3, "c2.xhtml", "Chapter 2", TocEntryKind::Unknown),
];
