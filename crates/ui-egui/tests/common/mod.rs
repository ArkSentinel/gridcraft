//! The shared strict test harness: bundled faces plus the pinned craft-fonts input, never the
//! host's installed fonts.
//!
//! `egui_kittest` keeps `MissingGlyphPolicy::Panic` and accessibility checks on. The production
//! font set probes system fonts for scripts a workstation may lack; the suite must not, or a
//! minimal CI image would fail for a reason unrelated to the change under test. `CRAFT_FONTS_DIR`
//! is the pinned input (never a font copy in this repository); without it only egui's bundled
//! faces are installed and the non-Latin language cases are skipped explicitly in the one test
//! that renders every catalog.

use egui_kittest::HarnessBuilder;
use gridcraft_engine::Session;
use gridcraft_ui_egui::{Services, SheetApp};

/// A kittest builder with the deterministic font set and GridCraft's visuals installed.
pub fn builder<State>() -> HarnessBuilder<State> {
    egui_kittest::Harness::builder().with_context_setup(|ctx| {
        ctx.set_fonts(gridcraft_ui_egui::theme::pinned_font_definitions());
        gridcraft_ui_egui::theme::apply(ctx, false);
    })
}

/// A test app whose first `logic` pass keeps the harness's pinned font set.
pub fn app(session: Session) -> SheetApp {
    prepare(SheetApp::new(session, Services::default()))
}

/// Mark `app` as already using the harness's pinned font set.
pub fn prepare(mut app: SheetApp) -> SheetApp {
    app.use_prepared_fonts();
    app
}
