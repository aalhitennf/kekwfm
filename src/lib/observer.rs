use std::{
    path::Path,
    sync::Mutex,
    sync::mpsc::{channel, Sender, Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};


use eframe::egui::Context;
use notify::{
    watcher, DebouncedEvent, Error as NotifyError, RecommendedWatcher, RecursiveMode, Watcher,
};
use crossbeam_channel::{Sender as CbSender, Receiver as CbReceiver, unbounded};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Pappa(String);


// pub static KEKW: OnceCell<Mutex<Pappa>> = OnceCell::new();

// impl Pappa {
//     // pub fn global() -> &'static Pappa {
//     //     &KEKW.get().unwrap().lock().unwrap()
//     // }
//     pub fn set_value(&mut self, val: &str) {
//         self.0 = String::from(val);
//     }
//     pub fn get_value(&self) -> &str {
//         &self.0
//     }
// }


pub struct FsObserver {
    // sender: Sender<DebouncedEvent>,
    pub receiver: CbReceiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
    handle: JoinHandle<()>,
    path: String,
}

impl FsObserver {
    pub fn new(path: &str, ctx: Context) -> Self {

        let (tx, rx) = std::sync::mpsc::channel();

        let (handle, receiver) = spawn_observer_thread(rx, ctx);

        let mut watcher = watcher(tx.clone(), Duration::from_millis(100)).unwrap();

        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

        FsObserver {
            // sender: tx,
            receiver,
            watcher,
            handle,
            path: path.to_string(),
        }
    }
    pub fn initialize(&mut self, path: &str) {
        if !Path::new(&path).is_dir() {
            println!("path is not directory: {}", path);
            return;
        }

        self.path = path.to_string();
        self.watcher
            .watch(&path, RecursiveMode::NonRecursive)
            .unwrap();
        println!("Observer initialized: {path}");
    }

    // Unwatch the current path and switch to new given path
    pub fn change_path(&mut self, path: &str) {
        if !Path::new(&path).is_dir() {
            println!("path is not directory: {}", path);
            return;
        }

        if path != &self.path {
            // This can fail i.e. if folder we was watching got deleted
            if let Err(e) = self.watcher.unwatch(&self.path) {
                println!("Failed to unwatch {}", &self.path);
                println!("{e:?}");
            } else {
                println!("Observer unwached: {}", &self.path);
            }

            // Even if unwatch errored, we can continue normally

            self.path = path.to_string();
            println!("Current path: {}", self.path);

            if let Err(e) = self.watcher.watch(&self.path, RecursiveMode::NonRecursive) {
                println!("Failed to watch path {}", self.path);
                println!("{e:?}");
            } else {
                println!("Observing path: {path}");
            }
        } else {
            println!("Path didn't change, skipped");
        }
    }
}

// impl Drop for FsObserver {
//     fn drop(&mut self) {
//         // if let Some(o) = self.inner.take() {
//         self.sender
//             .send(ObserverEvent::Terminate)
//             .unwrap();
//         self.handle.join().unwrap();
//         // }
//     }
// }

// #[derive(Serialize)]
// struct ObserverEvent {
//     event: String,
//     path: String,
//     error_type: Option<String>,
//     error_message: Option<String>,
// }

// impl From<DebouncedEvent> for ObserverEvent {
//     fn from(event: DebouncedEvent) -> Self {
//         match event {
//             DebouncedEvent::Chmod(path) =>
//         }
//     }
// }

// type IObserverEvent = DebouncedEvent;

// impl ToString for ObserverEvent {
//     fn to_string(&self) -> String {
//         match self {
//             DebouncedEvent::Chmod(path)
//         }
//     }
// }

// Match and parse observer events into serialized objects for gui
// fn handle_observer_event(event: DebouncedEvent) {
//     println!("{:?}", event);
// }

// Creates channel and spawns a thread. Give channel reveiver to the thread and
// return sender with thread handle

// pub enum ObserverEvent {
//     FsEvent(DebouncedEvent),
//     Terminate,
// }

fn spawn_observer_thread(
    rx: Receiver<DebouncedEvent>,
    ctx: Context,
) -> (JoinHandle<()>, CbReceiver<DebouncedEvent>) {
    let (cbtx, cbrx) = crossbeam_channel::unbounded();
    let handle = thread::spawn(move || {
        loop {
            if let Ok(event) = rx.recv() {
                cbtx.send(event).unwrap();
                ctx.request_repaint();

            }
        }
    });

    (handle, cbrx)
}
