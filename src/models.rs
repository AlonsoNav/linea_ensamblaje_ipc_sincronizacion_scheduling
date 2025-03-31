use std::time::{Duration, Instant, SystemTime};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]  // Add Debug trait for easier debugging
pub struct Product {
    pub id: usize,
    pub arrival_time: SystemTime,
    pub processing_steps: Vec<ProcessingStep>,
}

#[derive(Debug)]
pub struct ProcessingStep {
    pub entry_time: Option<SystemTime>,
    pub exit_time: Option<SystemTime>,
}

#[derive(Clone)]
pub struct Station {
    name: String,
    processing_time: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Product>>>,
    sender: Option<mpsc::Sender<Product>>,
}

impl Station {
    pub fn new(
        name: &str,
        processing_time: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Product>>>,
        sender: Option<mpsc::Sender<Product>>,
    ) -> Self {
        Station {
            name: name.to_string(),
            processing_time,
            receiver,
            sender,
        }
    }

    pub fn set_sender(&mut self, sender: mpsc::Sender<Product>) {
        self.sender = Some(sender);
    }

    pub fn start(&self) {
        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.run();
        });
    }

    fn run(&self) {
        loop {
            let product = match self.receiver.lock().unwrap().recv() {
                Ok(p) => p,
                Err(_) => break,
            };
            
            println!("Estación {} procesando producto {}", self.name, product.id);
            thread::sleep(Duration::from_millis(self.processing_time as u64)); // Simulate processing time
            self.sender.as_ref().unwrap().send(product).unwrap();
        }
    }
}
