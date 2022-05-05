use eframe::{egui::{Context, TopBottomPanel, Frame, style::Margin, Layout, TextFormat}, epaint::{Color32, text::{LayoutJob, TextWrapping}}, emath::Align};
use kekwlib::dirutils::DirectoryListingMetaData;

const MARGIN: f32 = 10.0;

pub struct StatusBar {
    frame: Frame,
}

impl StatusBar {
    pub fn new(fill: Color32) -> Self {
        let frame = Frame {
            inner_margin: Margin::same(MARGIN),
            fill,
            ..Frame::default()
        };
        StatusBar { frame }
    }
    pub fn show(&mut self, ctx: &Context, selected_items: usize, metadata: &DirectoryListingMetaData) {
        let wrap = TextWrapping {
            max_rows: 1,
            ..TextWrapping::default()
        };

        TopBottomPanel::bottom("status_bar")
            .resizable(false)
            .max_height(40.0)
            .frame(self.frame)
            .show(ctx, |ui| {
                ui.separator();
                ui.columns(2, |ui| {

                    let mut job1 = LayoutJob::single_section(format!("{} items selected", selected_items), TextFormat::default());
                    job1.wrap = wrap.clone();

                    ui[0].label(job1);
        
                    let mut job2 = LayoutJob::single_section(format!("{}", metadata), TextFormat::default());
                    job2.wrap = wrap;

                    ui[1].with_layout(Layout::top_down_justified(Align::Max).with_main_wrap(false), |ui| {
                        ui.label(job2);
                    });
                });
            });
    }
}
