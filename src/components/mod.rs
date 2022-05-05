pub mod context_menu;

pub mod grid_view;
mod left_panel;
mod list_view;
mod statusbar;
mod top_panel;

use kekwlib::dirutils::{DirectoryListingItem, ReadDirOptions};
// pub use grid_view;
pub use left_panel::LeftPanel;
pub use list_view::ListView;
pub use statusbar::StatusBar;
pub use top_panel::TopPanel;

use crate::textures::Textures;

pub trait DirectoryView {
    fn show(&mut self, items: &mut [DirectoryListingItem], textures: &Textures, read_dir_options: &mut ReadDirOptions);
}
