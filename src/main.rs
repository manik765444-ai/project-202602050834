use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use std::{thread, process, path::Path};

fn main() {
    // The directory to watch
    let path_to_watch = "./"; // Use the current directory as an example
    let (tx, rx) = channel();

    // Create a file system watcher
    let mut fs_watcher = match watcher(tx, Duration::from_secs(2)) {
        Ok(watcher) => watcher,
        Err(e) => {
            eprintln!("Error: Failed to initialize the file watcher: {}", e);
            process::exit(1);
        }
    };

    // Add the path to be watched
    if let Err(e) = fs_watcher.watch(path_to_watch, RecursiveMode::Recursive) {
        eprintln!("Error: Failed to watch the path '{}': {}", path_to_watch, e);
        process::exit(1);
    }

    println!("Watching for changes in '{}'", path_to_watch);

    // Run a thread to handle events
    if let Err(e) = handle_events(rx) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

// Function to handle incoming events
fn handle_events(rx: Receiver<DebouncedEvent>) -> Result<(), String> {
    thread::spawn(move || {
        // Listen for events in a loop
        while let Ok(event) = rx.recv() {
            // Handle the event
            match event {
                DebouncedEvent::Create(path) => {
                    println!("File created: {}", path.display());
                }
                DebouncedEvent::Write(path) => {
                    println!("File modified: {}", path.display());
                }
                DebouncedEvent::Remove(path) => {
                    println!("File deleted: {}", path.display());
                }
                DebouncedEvent::Rename(src, dest) => {
                    println!("File renamed from '{}' to '{}'", src.display(), dest.display());
                }
                DebouncedEvent::Rescan => {
                    println!("Rescan required");
                }
                DebouncedEvent::Error(err, maybe_path) => {
                    if let Some(path) = maybe_path {
                        eprintln!("Error '{}' on path '{}'", err, path.display());
                    } else {
                        eprintln!("Error: {}", err);
                    }
                }
                _ => {
                    println!("Unhandled event: {:?}", event);
                }
            }
        }
    }).join().map_err(|_| "Failed to join event handler thread".to_string())?;
    Ok(())
}