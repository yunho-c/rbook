use rbook::Epub;
use rbook::epub::EpubOpenOptions;
use std::io::Cursor;
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Once;

pub const EPUB3_DIR: &str = "tests/ebooks/example_epub";
const EPUB3_RELAXED: &str = "tests/ebooks/epub3_relaxed";
const EPUB2_DIR: &str = "tests/ebooks/epub2";
const EPUB3_MULTI_NAV_DIR: &str = "tests/ebooks/fixtures/epub3_multi_nav";
const EPUB3_DEEP_NESTING_DIR: &str = "tests/ebooks/fixtures/epub3_deep_nesting";
const EPUB3_EDGE_HREFS_DIR: &str = "tests/ebooks/fixtures/epub3_edge_hrefs";
const EPUB2_NCX_NESTED_DIR: &str = "tests/ebooks/fixtures/epub2_ncx_nested";

const EPUB3_FILE_BYTES: &[u8] = include_bytes!("../../tests/ebooks/example.epub");

#[cfg(not(target_arch = "wasm32"))]
static ENSURE_FIXTURES: Once = Once::new();

pub enum TestEpub {
    /// Unzipped Epub `2` + `3` directory
    ///
    /// Mapped to: [`EPUB3_DIR`]
    Epub3Dir,
    /// Zipped Epub `2` + `3` File
    ///
    /// Mapped to: `tests/ebooks/example.epub`
    Epub3File,
    /// Zipped malformed Epub `2` + `3` File
    ///
    /// Intended to for relaxed parsing (`strict` mode disabled).
    ///
    /// Mapped to: [`EPUB3_RELAXED`]
    Epub3Relaxed,
    /// Unzipped Epub `2` directory
    ///
    /// Mapped to: [`EPUB2_DIR`]
    Epub2Dir,
    /// Unzipped EPUB 3 fixture with multiple navs (toc, landmarks, page-list).
    ///
    /// Mapped to: [`EPUB3_MULTI_NAV_DIR`]
    Epub3MultiNavDir,
    /// Unzipped EPUB 3 fixture with deep toc nesting.
    ///
    /// Mapped to: [`EPUB3_DEEP_NESTING_DIR`]
    Epub3DeepNestingDir,
    /// Unzipped EPUB 3 fixture with edge-case hrefs.
    ///
    /// Mapped to: [`EPUB3_EDGE_HREFS_DIR`]
    Epub3EdgeHrefsDir,
    /// Unzipped EPUB 2 fixture with nested NCX navPoints.
    ///
    /// Mapped to: [`EPUB2_NCX_NESTED_DIR`]
    Epub2NcxNestedDir,
}

impl TestEpub {
    pub fn open(self) -> Epub {
        self.build(|b| b)
    }

    pub fn build(self, builder: impl Fn(EpubOpenOptions) -> EpubOpenOptions) -> Epub {
        let options = builder(EpubOpenOptions::new());

        // File bytes
        if matches!(self, Self::Epub3File) {
            let cursor = Cursor::new(EPUB3_FILE_BYTES);
            options.read(cursor).unwrap()
        }
        // Directory
        else {
            if matches!(
                self,
                Self::Epub3MultiNavDir
                    | Self::Epub3DeepNestingDir
                    | Self::Epub3EdgeHrefsDir
                    | Self::Epub2NcxNestedDir
            ) {
                ensure_fixtures();
            }
            options
                .open(match self {
                    Self::Epub3Dir => EPUB3_DIR,
                    Self::Epub3Relaxed => EPUB3_RELAXED,
                    Self::Epub2Dir => EPUB2_DIR,
                    Self::Epub3MultiNavDir => EPUB3_MULTI_NAV_DIR,
                    Self::Epub3DeepNestingDir => EPUB3_DEEP_NESTING_DIR,
                    Self::Epub3EdgeHrefsDir => EPUB3_EDGE_HREFS_DIR,
                    Self::Epub2NcxNestedDir => EPUB2_NCX_NESTED_DIR,
                    _ => panic!("Unexpected test file provided"),
                })
                .unwrap()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_fixtures() {
    let fixture_paths = [
        EPUB3_MULTI_NAV_DIR,
        EPUB3_DEEP_NESTING_DIR,
        EPUB3_EDGE_HREFS_DIR,
        EPUB2_NCX_NESTED_DIR,
    ];

    let fixtures_missing = fixture_paths
        .iter()
        .any(|path| !Path::new(path).exists());
    if !fixtures_missing {
        return;
    }

    ENSURE_FIXTURES.call_once(|| {
        let status = Command::new("python3")
            .arg("scripts/gen_epub_fixtures.py")
            .status()
            .expect("Failed to run scripts/gen_epub_fixtures.py");
        if !status.success() {
            panic!("Fixture generation failed; run `python3 scripts/gen_epub_fixtures.py`.");
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn ensure_fixtures() {}
