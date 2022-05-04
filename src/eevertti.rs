use crossbeam_channel::Sender;
use kekwlib::dirutils::DirectoryListingItem;
use once_cell::sync::OnceCell;

static EEVERTTI: OnceCell<Sender<KekEvent>> = OnceCell::new();

#[derive(Debug)]
pub enum MouseButton {
    Back,
    Forward,
}

#[derive(Debug)]
pub enum KekEvent {
    Print(String),
    Navigate(String),
    NavigateParent,
    NavigateBack,
    NavigateForward,
    RefreshDirList,
    XdgOpenFile(String),
    DeleteFile(String),
    DeleteFolder(String),
    DeleteSelected,
    FavouriteFolder(DirectoryListingItem),
    ButtonPress(MouseButton),
}

pub fn send_event(event: KekEvent) {
    if let Some(eevertti) = EEVERTTI.get() {
        if let Err(e) = eevertti.send(event) {
            println!("Failed to send event: {e:?}");
        }
    } else {
        println!("Failed to send event: eevertti not initialized");
    }
}

pub fn set_eevertti(tx: Sender<KekEvent>) {
    if let Err(e) = EEVERTTI.set(tx) {
        println!("Failed to create Eevertti: {e:?}");
    }
}
