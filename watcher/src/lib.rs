mod events;
mod processing;
#[cfg(unix)]
mod unix;

use std::io::Result as IoResult;
use std::str;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub use processing::ObservationEvent;

pub struct Watcher {
    file_descriptor: i32,
    watch_descriptor: i32,
    receiver: Option<Receiver<()>>,
    sender: Sender<()>,
    handle: Option<JoinHandle<()>>,
}

impl Watcher {
    pub fn new(directory: &str) -> IoResult<Watcher> {
        let file_descriptor = unix::observation_init()?;
        let (sender, receiver) = mpsc::channel();

        match unix::add_watch(file_descriptor, directory) {
            Ok(watch_descriptor) => Ok(Watcher {
                file_descriptor,
                watch_descriptor,
                receiver: Some(receiver),
                sender,
                handle: None,
            }),
            Err(error) => {
                unsafe { unix::close(file_descriptor) };
                Err(error)
            }
        }
    }

    pub fn start_observation(&mut self) -> Result<Receiver<ObservationEvent>, String> {
        let stop_receiver = match self.receiver.take() {
            Some(receiver) => receiver,
            None => return Err(String::from("Already observing.")),
        };
        let file_descriptor = self.file_descriptor;

        let (event_sender, event_receiver) = mpsc::channel();

        self.handle = Some(thread::spawn(move || {
            processing::observe(stop_receiver, event_sender, file_descriptor)
        }));
        Ok(event_receiver)
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        self.sender.send(()).unwrap();
        unsafe {
            unix::inotify_rm_watch(self.file_descriptor, self.watch_descriptor);
            unix::close(self.file_descriptor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Error;
    use std::time::Duration;

    #[test]
    fn new_creates_instance() {
        Watcher::new(".").unwrap_or_else(|err| {
            panic!("Instance was not created with error {}", err);
        });
    }
    #[test]
    fn drops_loop() -> Result<(), Error> {
        {
            let mut watcher = Watcher::new(".")?;
            let _receiver = watcher.start_observation().unwrap();
            File::create("touch").unwrap();

            thread::sleep(Duration::from_millis(10));
        }
        thread::sleep(Duration::from_millis(10));

        fs::remove_file("touch").unwrap();
        Ok(())
    }
}
