//! Help ▸ About: the framework's hosted About dialog over the app's info and the baked credits.

use craft::behavior::IconId;
use craft::ui::patterns::form_dialogs::{AboutInfo, AboutLink, AboutTab, HostedAbout};

use crate::PdfCraftApp;

/// What the About tab shows: PdfCraft's name, blurb and version, where this build comes from, and its links.
/// The app's name is plain text in the title; no mark is drawn.
fn info() -> AboutInfo {
    let blurb =
        format!("{}\n{}", tl!("A clean-room, open-source PDF application written in Rust. MIT OR Apache-2.0."), pdfcraft_engine::links::BASED_ON);
    let info = AboutInfo::new("PdfCraft", blurb, pdfcraft_engine::build_info::version())
        .built_with("Rendering: hayro (bootstrap) · UI: egui · Icons: Lucide (ISC) · Fonts: Inter, JetBrains Mono, Dancing Script (OFL)");
    info.links(pdfcraft_engine::links::LINKS.iter().map(|l| AboutLink::new(tl!(l.label).to_string(), l.url).icon(IconId::new(l.icon))))
}

impl PdfCraftApp {
    /// Opens the About window on `tab` (Help ▸ About, `help.about`).
    pub(crate) fn open_about(&mut self, tab: AboutTab) {
        crate::dialogs::host::open(self, HostedAbout::new(info(), crate::credits_data::credits()).tab(tab));
    }

    /// Opens the link the user activated in the About window, the way the `help.*` link commands do.
    pub(crate) fn about_pass(&mut self, _ctx: &egui::Context) {
        let Some(url) = self.dialogs.get_mut::<HostedAbout>().and_then(HostedAbout::take_link) else {
            return;
        };
        self.open_url(&url);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn about_shows_the_app_name_as_plain_text_and_where_the_build_comes_from() {
        let info = super::info();
        assert_eq!(info.app_name.as_str(), "PdfCraft");
        assert!(info.mark.is_none() && info.part_of.is_none() && info.community.is_none(), "no mark, logo or community button");
        assert!(info.blurb.as_str().contains(pdfcraft_engine::links::BASED_ON), "{}", info.blurb.as_str());
        assert!(info.links.iter().all(|l| l.url.starts_with(pdfcraft_engine::links::GITHUB)), "every link goes to the upstream repository");
    }

    #[test]
    fn about_links_keep_their_icons_and_destinations() {
        let info = super::info();
        assert_eq!(info.links.len(), pdfcraft_engine::links::LINKS.len());
        for (shown, expected) in info.links.iter().zip(pdfcraft_engine::links::LINKS) {
            assert_eq!(shown.url, expected.url);
            assert_eq!(shown.icon, Some(craft::behavior::IconId::new(expected.icon)));
        }
    }
}
