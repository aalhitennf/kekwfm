use std::{
    path::Path,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::Duration,
};

use crossbeam_channel::{unbounded, Receiver as CbReceiver};
use eframe::egui::Context;
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

pub struct FsObserver {
    pub receiver: CbReceiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
    #[allow(unused)]
    handle: JoinHandle<()>,
    pub path: String,
}

impl FsObserver {
    pub fn new<P: AsRef<Path> + ToString + Copy>(path: P, ctx: Context) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let (handle, receiver) = spawn_observer_thread(rx, ctx);

        let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

        FsObserver {
            // sender,
            receiver,
            watcher,
            handle,
            path: path.to_string(),
        }
    }

    // Unwatch the current path and switch to new given path
    pub fn change_path<P: AsRef<Path> + ToString>(&mut self, path: P) {
        if let Err(e) = self.watcher.unwatch(&self.path) {
            println!("{e:?}");
        }
        // } else {
        //     println!("Observer unwached: {}", self.path);
        // }

        // Even if unwatch errored, we can continue normally

        self.path = path.to_string();
        // println!("Current path: {}", self.path);

        if let Err(e) = self.watcher.watch(&self.path, RecursiveMode::NonRecursive) {
            println!("Failed to watch path {}", self.path);
            println!("{e:?}");
        }
        // } else {
        //     println!("Observing path: {}", self.path);
        // }
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

// pub enum ObserverEvent {
//     FsEvent(DebouncedEvent),
//     Terminate,
// }

fn spawn_observer_thread(
    rx: Receiver<DebouncedEvent>,
    ctx: Context,
) -> (JoinHandle<()>, CbReceiver<DebouncedEvent>) {
    let (cbtx, cbrx) = unbounded();
    let handle = thread::spawn(move || loop {
        if let Ok(event) = rx.recv() {
            cbtx.send(event).unwrap();
            ctx.request_repaint();
        }
    });

    (handle, cbrx)
}
