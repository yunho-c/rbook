#!/usr/bin/env python3
from __future__ import annotations

import pathlib
import textwrap
import zipfile
import shutil


ROOT = pathlib.Path(__file__).resolve().parents[1]
FIXTURES_DIR = ROOT / "tests" / "ebooks" / "fixtures"


def write_text(path: pathlib.Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write(content)


def container_xml(package_path: str) -> str:
    return textwrap.dedent(
        f"""\
        <?xml version="1.0" encoding="UTF-8"?>
        <container version="1.0"
            xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
          <rootfiles>
            <rootfile full-path="{package_path}"
                media-type="application/oebps-package+xml"/>
          </rootfiles>
        </container>
        """
    )


def build_zip(src_dir: pathlib.Path, zip_path: pathlib.Path) -> None:
    if zip_path.exists():
        zip_path.unlink()

    with zipfile.ZipFile(zip_path, "w") as archive:
        mimetype = src_dir / "mimetype"
        archive.write(mimetype, "mimetype", compress_type=zipfile.ZIP_STORED)
        for path in sorted(src_dir.rglob("*")):
            if path.is_dir() or path.name == "mimetype":
                continue
            rel = path.relative_to(src_dir).as_posix()
            archive.write(path, rel, compress_type=zipfile.ZIP_DEFLATED)


def write_fixture(name: str, files: dict[str, str]) -> None:
    fixture_dir = FIXTURES_DIR / name
    if fixture_dir.exists():
        shutil.rmtree(fixture_dir)
    for rel, content in files.items():
        write_text(fixture_dir / rel, content)
    build_zip(fixture_dir, FIXTURES_DIR / f"{name}.epub")


def main() -> None:
    fixtures = {
        "epub3_multi_nav": {
            "mimetype": "application/epub+zip",
            "META-INF/container.xml": container_xml("EPUB/package.opf"),
            "EPUB/package.opf": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <package xmlns="http://www.idpf.org/2007/opf"
                    unique-identifier="uid" version="3.0">
                  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
                    <dc:identifier id="uid">urn:uuid:multi-nav</dc:identifier>
                    <dc:title>Fixture - EPUB3 Multi Nav</dc:title>
                    <dc:language>en</dc:language>
                  </metadata>
                  <manifest>
                    <item id="nav" href="nav.xhtml"
                        media-type="application/xhtml+xml" properties="nav"/>
                    <item id="cover" href="cover.xhtml"
                        media-type="application/xhtml+xml"/>
                    <item id="c1" href="c1.xhtml"
                        media-type="application/xhtml+xml"/>
                    <item id="c2" href="c2.xhtml"
                        media-type="application/xhtml+xml"/>
                  </manifest>
                  <spine>
                    <itemref idref="cover" linear="no"/>
                    <itemref idref="c1"/>
                    <itemref idref="c2"/>
                  </spine>
                </package>
                """
            ),
            "EPUB/nav.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml"
                    xmlns:epub="http://www.idpf.org/2007/ops">
                  <head>
                    <title>Navigation</title>
                  </head>
                  <body>
                    <nav epub:type="toc" id="toc">
                      <ol>
                        <li><a href="cover.xhtml">Cover</a></li>
                        <li><a href="c1.xhtml">Chapter 1</a></li>
                        <li>
                          <a href="c2.xhtml">Chapter 2</a>
                          <ol>
                            <li><a href="c2.xhtml#s1">Section 2.1</a></li>
                            <li><a href="c2.xhtml#s2">Section 2.2</a></li>
                          </ol>
                        </li>
                      </ol>
                    </nav>
                    <nav epub:type="landmarks" id="landmarks">
                      <ol>
                        <li>
                          <a epub:type="cover" href="cover.xhtml">Cover</a>
                        </li>
                        <li>
                          <a epub:type="bodymatter" href="c1.xhtml">Body</a>
                        </li>
                      </ol>
                    </nav>
                    <nav epub:type="page-list" id="page-list">
                      <ol>
                        <li><a href="c1.xhtml#p1">1</a></li>
                        <li><a href="c2.xhtml#p2">2</a></li>
                      </ol>
                    </nav>
                  </body>
                </html>
                """
            ),
            "EPUB/cover.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Cover</title></head>
                  <body>
                    <h1 id="cover">Cover</h1>
                  </body>
                </html>
                """
            ),
            "EPUB/c1.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Chapter 1</title></head>
                  <body>
                    <h1 id="p1">Chapter 1</h1>
                    <p id="s1">Section 1</p>
                  </body>
                </html>
                """
            ),
            "EPUB/c2.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Chapter 2</title></head>
                  <body>
                    <h1 id="p2">Chapter 2</h1>
                    <p id="s1">Section 2.1</p>
                    <p id="s2">Section 2.2</p>
                  </body>
                </html>
                """
            ),
        },
        "epub3_deep_nesting": {
            "mimetype": "application/epub+zip",
            "META-INF/container.xml": container_xml("EPUB/package.opf"),
            "EPUB/package.opf": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <package xmlns="http://www.idpf.org/2007/opf"
                    unique-identifier="uid" version="3.0">
                  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
                    <dc:identifier id="uid">urn:uuid:deep-nesting</dc:identifier>
                    <dc:title>Fixture - EPUB3 Deep Nesting</dc:title>
                    <dc:language>en</dc:language>
                  </metadata>
                  <manifest>
                    <item id="nav" href="nav.xhtml"
                        media-type="application/xhtml+xml" properties="nav"/>
                    <item id="c1" href="c1.xhtml"
                        media-type="application/xhtml+xml"/>
                  </manifest>
                  <spine>
                    <itemref idref="c1"/>
                  </spine>
                </package>
                """
            ),
            "EPUB/nav.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml"
                    xmlns:epub="http://www.idpf.org/2007/ops">
                  <head>
                    <title>Navigation</title>
                  </head>
                  <body>
                    <nav epub:type="toc" id="toc">
                      <ol>
                        <li>
                          <a href="c1.xhtml#l1">Level 1</a>
                          <ol>
                            <li>
                              <a href="c1.xhtml#l1-1">Level 2</a>
                              <ol>
                                <li>
                                  <a href="c1.xhtml#l1-1-1">Level 3</a>
                                  <ol>
                                    <li>
                                      <a href="c1.xhtml#l1-1-1-1">Level 4</a>
                                    </li>
                                  </ol>
                                </li>
                              </ol>
                            </li>
                          </ol>
                        </li>
                        <li><a href="c1.xhtml#l2">Another Root</a></li>
                      </ol>
                    </nav>
                  </body>
                </html>
                """
            ),
            "EPUB/c1.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Deep Nesting</title></head>
                  <body>
                    <h1 id="l1">Level 1</h1>
                    <h2 id="l1-1">Level 2</h2>
                    <h3 id="l1-1-1">Level 3</h3>
                    <h4 id="l1-1-1-1">Level 4</h4>
                    <h1 id="l2">Another Root</h1>
                  </body>
                </html>
                """
            ),
        },
        "epub3_edge_hrefs": {
            "mimetype": "application/epub+zip",
            "META-INF/container.xml": container_xml("EPUB/package.opf"),
            "EPUB/package.opf": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <package xmlns="http://www.idpf.org/2007/opf"
                    unique-identifier="uid" version="3.0">
                  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
                    <dc:identifier id="uid">urn:uuid:edge-hrefs</dc:identifier>
                    <dc:title>Fixture - EPUB3 Edge Hrefs</dc:title>
                    <dc:language>en</dc:language>
                  </metadata>
                  <manifest>
                    <item id="nav" href="nav.xhtml"
                        media-type="application/xhtml+xml" properties="nav"/>
                    <item id="c1" href="c1.xhtml"
                        media-type="application/xhtml+xml"/>
                    <item id="file-space" href="file%20name.xhtml"
                        media-type="application/xhtml+xml"/>
                  </manifest>
                  <spine>
                    <itemref idref="c1"/>
                    <itemref idref="file-space"/>
                  </spine>
                </package>
                """
            ),
            "EPUB/nav.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml"
                    xmlns:epub="http://www.idpf.org/2007/ops">
                  <head>
                    <title>Navigation</title>
                  </head>
                  <body>
                    <nav epub:type="toc" id="toc">
                      <ol>
                        <li><a href="c1.xhtml#p1">Chapter 1</a></li>
                        <li>
                          <a href="file%20name.xhtml#p2">File With Spaces</a>
                        </li>
                        <li><a href="#local">Local Fragment</a></li>
                      </ol>
                    </nav>
                    <section id="local">Local Target</section>
                  </body>
                </html>
                """
            ),
            "EPUB/c1.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Chapter 1</title></head>
                  <body>
                    <h1 id="p1">Chapter 1</h1>
                  </body>
                </html>
                """
            ),
            "EPUB/file name.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>File With Spaces</title></head>
                  <body>
                    <h1 id="p2">File With Spaces</h1>
                  </body>
                </html>
                """
            ),
        },
        "epub2_ncx_nested": {
            "mimetype": "application/epub+zip",
            "META-INF/container.xml": container_xml("OEBPS/content.opf"),
            "OEBPS/content.opf": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <package xmlns="http://www.idpf.org/2007/opf"
                    unique-identifier="uid" version="2.0">
                  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
                    <dc:identifier id="uid">urn:uuid:ncx-nested</dc:identifier>
                    <dc:title>Fixture - EPUB2 NCX Nested</dc:title>
                    <dc:language>en</dc:language>
                  </metadata>
                  <manifest>
                    <item id="ncx" href="toc.ncx"
                        media-type="application/x-dtbncx+xml"/>
                    <item id="c1" href="c1.xhtml"
                        media-type="application/xhtml+xml"/>
                    <item id="c2" href="c2.xhtml"
                        media-type="application/xhtml+xml"/>
                  </manifest>
                  <spine toc="ncx">
                    <itemref idref="c1"/>
                    <itemref idref="c2"/>
                  </spine>
                </package>
                """
            ),
            "OEBPS/toc.ncx": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <ncx xmlns="http://www.daisy.org/z3986/2005/ncx/"
                    version="2005-1">
                  <head>
                    <meta name="dtb:uid" content="urn:uuid:ncx-nested"/>
                    <meta name="dtb:depth" content="2"/>
                    <meta name="dtb:totalPageCount" content="0"/>
                    <meta name="dtb:maxPageNumber" content="0"/>
                  </head>
                  <docTitle><text>NCX Nested</text></docTitle>
                  <navMap>
                    <navPoint id="np1" playOrder="2">
                      <navLabel><text>Chapter 1</text></navLabel>
                      <content src="c1.xhtml"/>
                      <navPoint id="np1-1" playOrder="1">
                        <navLabel><text>Chapter 1.1</text></navLabel>
                        <content src="c1.xhtml#s1"/>
                      </navPoint>
                    </navPoint>
                    <navPoint id="np2" playOrder="3">
                      <navLabel><text>Chapter 2</text></navLabel>
                      <content src="c2.xhtml"/>
                    </navPoint>
                  </navMap>
                </ncx>
                """
            ),
            "OEBPS/c1.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Chapter 1</title></head>
                  <body>
                    <h1>Chapter 1</h1>
                    <p id="s1">Section 1.1</p>
                  </body>
                </html>
                """
            ),
            "OEBPS/c2.xhtml": textwrap.dedent(
                """\
                <?xml version="1.0" encoding="utf-8"?>
                <html xmlns="http://www.w3.org/1999/xhtml">
                  <head><title>Chapter 2</title></head>
                  <body>
                    <h1>Chapter 2</h1>
                  </body>
                </html>
                """
            ),
        },
    }

    FIXTURES_DIR.mkdir(parents=True, exist_ok=True)
    for name, files in fixtures.items():
        write_fixture(name, files)


if __name__ == "__main__":
    main()
