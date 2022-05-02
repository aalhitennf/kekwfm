use std::{
    path::Path,
    sync::mpsc::{channel, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use notify::{
    watcher, DebouncedEvent, Error as NotifyError, RecommendedWatcher, RecursiveMode, Watcher,
};
// use tauri::Window;

// use crate::KekwState;

pub struct FsObserver {
    sender: Sender<DebouncedEvent>,
    watcher: RecommendedWatcher,
    handle: JoinHandle<()>,
    path: String,
}

impl FsObserver {
    pub fn new(path: &str) -> Self {
        let (tx, handle) = spawn_observer_thread();

        FsObserver {
            sender: tx.clone(),
            watcher: watcher(tx, Duration::from_millis(100)).unwrap(),
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

impl Drop for Observer {
    fn drop(&mut self) {
        if let Some(o) = self.inner.take() {
            o.sender
                .send(DebouncedEvent::Error(
                    NotifyError::Generic(String::from("TERMINATE_OBSERVER")),
                    None,
                ))
                .unwrap();
            o.handle.join().unwrap();
        }
    }
}

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

// TODO ÄLÄ KÄYTÄ TÄTÄ, LPUTON LOOP
fn spawn_observer_thread() -> (Sender<DebouncedEvent>, JoinHandle<()>) {
    let (tx, rx) = channel();

    let tx_c = tx.clone();
    let handle = thread::spawn(move || {
        loop {
            if let Ok(event) = rx.recv() {
                match event {
                    // "Custom error" with message "TERMINATE_OBSERVER" to kill the thread gracefully
                    // Hacky but it works and proud of it
                    DebouncedEvent::Error(NotifyError::Generic(message), None) => {
                        if message == "TERMINATE_OBSERVER" {
                            println!("TERMINATE RECEIVED ({:?})", thread::current().id());
                            break;
                        } else {
                            // println!("Error::NotifyError: {}", message);
                            if let Err(e) = tx_c
                                .send(DebouncedEvent::Error(NotifyError::Generic(message), None))
                            {
                                println!("Failed to send message to watcher")
                            }
                        }
                    }
                    _ => {
                        // handle_observer_event(event);
                        // let e = format!("{:?}", event);
                        // window.emit("observer_event", e).unwrap();
                        tx_c.send(event);
                    }
                }
            }
        }
    });

    (tx, handle)
}

pub struct Observer {
    pub inner: Option<FsObserver>,
}
