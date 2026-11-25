# KatWork fork of PdfCraft

This repository is the KatWork fork of PdfCraft (https://github.com/storytold/pdfcraft). KatWork builds the app's CLI and web
build from this fork. It carries the patches listed below; everything else is upstream code.

Based on PdfCraft by the ArtCraft team (https://github.com/storytold/pdfcraft). This build is not an ArtCraft product.

## Fork base

- Fork base (this fork's `main` when the KatWork work started): `6e910675c68dd5092e889065cfe8d0274c1b2742`
- Upstream `storytold/pdfcraft` head at that time: `6e910675c68dd5092e889065cfe8d0274c1b2742`

## KatWork patches

| Patch | Upstream PR |
|---|---|
| `chore(katwork): remove the ArtCraft marks` | not proposed upstream (the brand terms let a fork that proposes changes keep the marks; this one is a distributed modified version) |
| `feat(katwork): js feature, on by default` | draft, not opened (in the fork PR body) |

## The `js` gate

`pdfcraft-js` (Acrobat JavaScript on the boa engine, and the FormCalc interpreter) runs a document's field, document and XFA
scripts. It is an optional dependency of `pdfcraft-engine` behind the feature `js`, on by default, so an upstream build is
unchanged. The workspace inherits `pdfcraft-engine` and `pdfcraft-ui-egui` with their defaults off, and `pdfcraft-automation`,
`pdfcraft-ui-egui`, `pdfcraft-cli` (`default = ["mcp", "js"]`), `pdfcraft-web` (a new `[features]` table) and the desktop app
forward `js` by default. Without it the engine compiles a stand-in (`crates/engine/src/js_off/`) that reads the script engine's
data types from its own model files and runs nothing: JavaScript stays off whatever the preference says, no script runs when a
document opens or changes (fields keep their stored values; calculate, validate and format events do nothing), and `js_run`,
`js_set_document_script` and `form_set_script` answer "JavaScript is not available in this build" and store nothing. The
KatWork build is `pdfcraft-cli --no-default-features --features mcp` and `pdfcraft-web --no-default-features`.

`scripts/katwork-profile.sh` (CI job `katwork-profile`) checks that `pdfcraft-js` and `boa_engine` are out of the CLI and web
build graphs with the gate off and in them with the defaults; `crates/engine/tests/katwork_profile.rs` and
`crates/automation/tests/katwork_profile.rs` run with it off.

## The marks

The ArtCraft name, wordmark and logos are trademarks of the ArtCraft Team and are not covered by the app's licence. This fork
removes them, as the brand terms require of a distributed modified version: `docs/brand/` is deleted; links to the ArtCraft
website and its community (Discord), and taglines that present the build as an ArtCraft product, are removed; help links and
`app.links`-style results point to the upstream repository (https://github.com/storytold/pdfcraft); README.md, NOTICE and the
About window carry the "Based on" line above as plain text. No new logo is drawn: where a mark was shown, the app's name is shown
as plain text.

`scripts/check-no-marks.sh` (CI job `no-marks` in `.github/workflows/katwork.yml`) fails when a tracked file is a copy of a
fork-base brand file (`scripts/katwork-marks/hashes`), when a line names the marks or the ArtCraft website, when a line names
ArtCraft and is not in `scripts/katwork-marks/allow`, or when an image below is missing from this file.

### What still names ArtCraft, and why (`scripts/katwork-marks/allow`)

- Copyright notices (`LICENSE-MIT`, `NOTICE`, README): required by the licences.
- Application ids `ai.storyteller.pdfcraft` and installer paths and folders (`Programs\ArtCraft\...`): kept unchanged; they are
  identifiers, not marks, and changing them would break upgrades of existing installs.
- The ArtCraft Launcher's folders and the Flatpak permission for its version folder (`xdg-data/ArtCraft/versions`): they name
  a separate program, the upstream nightly builds' updater; KatWork builds no nightly.
- The upstream release workflow's secret name (`release.yml`) and the Windows ARM64 workflow's install path: upstream CI
  configuration, unchanged; KatWork does not run it.
- The app icon's provenance: `assets/app-icon/LICENSE.txt`, `assets/app-icon/README.md` and the icon rows of
  `ATTRIBUTION.toml` / `ATTRIBUTION.md` say the project owner drew it in ArtCraft (the tool it was drawn with). This is a
  record of where the artwork came from, not a mark, and the licence text is kept as written.
- The "Based on" line in code (`crates/engine/src/links.rs`, `BASED_ON`, which the About dialog shows) and in
  `packaging/macos/Credits.rtf`.

## Image assets of the fork base

Removed (the brand files, deleted with `docs/brand/`):

- `docs/brand/LICENSE-brand.txt`: removed
- `docs/brand/artcraft-logo-white.png`: removed
- `docs/brand/artcraft-logo-white.svg`: removed
- `docs/brand/artcraft-logo.png`: removed
- `docs/brand/artcraft-logo.svg`: removed
- `docs/brand/artcraft-mark-black.png`: removed
- `docs/brand/artcraft-mark-black.svg`: removed
- `docs/brand/artcraft-mark.png`: removed
- `docs/brand/artcraft-mark.svg`: removed

Every other image of the fork base, each kept:

- `assets/app-icon/hicolor/128x128/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/16x16/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/24x24/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/256x256/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/32x32/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/48x48/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/512x512/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/64x64/apps/ai.storyteller.pdfcraft.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/hicolor/scalable/apps/ai.storyteller.pdfcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft-1024.png`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft-document.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft-document.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft-small.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft.icns`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft.ico`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/app-icon/pdfcraft.svg`: kept: original artwork, licence in `assets/app-icon/LICENSE.txt` (viewed: no ArtCraft mark)
- `assets/icons/a-large-small.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/accessibility.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/arrow-up-right.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/badge-check.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/bell.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/book-open.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/bookmark-plus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/braces.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/camera.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/check-circle-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/circle-check.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/circle-help.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/circle-x.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/clipboard-paste.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/clock-3.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/cloud-upload.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/code-xml.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/columns-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/columns-3.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/copy-plus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/ellipsis-vertical.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-check.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-code.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-down.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-input.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-lock-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-minus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-output.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-pen-line.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-plus-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-scan.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-search.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-spreadsheet.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-symlink.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-text.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/file-x.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/files.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/film.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/filter.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/flip-horizontal-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/flip-vertical-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/folder-plus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/git-compare.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/hash.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/heading.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/highlighter.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/image-plus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/languages.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/list-checks.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/maximize.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/message-circle-reply.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/message-square-plus.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/message-square-quote.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/message-square-text.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/messages-square.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/mic.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/minimize-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/minimize.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/move-horizontal.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/move-right.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/panel-left.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/panel-right.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/paperclip.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/pen-line.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/presentation.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/printer.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/replace.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/reply.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/rows-3.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/scan-text.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/send.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/settings-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/share-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/shield-check.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/signature.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/square-dashed-mouse-pointer.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/square-terminal.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/squares-subtract.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/sticky-note.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/tag.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/text-cursor-input.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/text-select.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/trash-2.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/triangle-alert.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/type.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/user-round.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `assets/icons/wrench.svg`: kept: UI icon from Lucide (ISC), listed in ATTRIBUTION.toml (no ArtCraft mark; `scripts/check-no-marks.sh` reads its source)
- `docs/images/pdfcraft-bookmarks.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-dark.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-find.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-forms.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-layers.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-ocr-verify.png`: kept as is: screenshot of the upstream build's OCR Verify screen (viewed: no title bar, no ArtCraft mark or Discord button)
- `docs/images/pdfcraft-organize.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-palette.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-properties.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-scripts.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-split.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-tools.png`: changed: the upstream screenshot showed the home screen's "Join the ArtCraft community" card with the ArtCraft mark and the Discord buttons, and the title bar's Discord button; it is cropped to the All tools panel (no title bar), which shows neither (viewed)
- `docs/images/pdfcraft-twoup.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `docs/images/pdfcraft-viewer.png`: changed: screenshot of the upstream build with its title bar cropped off (the title bar showed the Discord button; the rest shows no ArtCraft mark; viewed)
- `packaging/macos/dmg/background.svg`: kept: macOS disk-image background, original artwork (the app name and icon; source read: no ArtCraft mark)
- `packaging/macos/dmg/background.tiff`: kept: macOS disk-image background (the app name and icon; viewed: no ArtCraft mark)
- `vendor/vello_common/assets/probe.png`: kept: test image of the vendored vello_common crate (viewed: no ArtCraft mark)
