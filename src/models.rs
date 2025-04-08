use std::time::{Duration, SystemTime};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy)]
pub enum SchedulingAlgorithm {
    FCFS,
    RoundRobin,
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: usize,
    pub arrival_time: SystemTime,
    pub processing_steps: Vec<ProcessingStep>,
    pub remaining_time: Option<usize>,
}

#[derive(Debug, Clone)]
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
    scheduling_algorithm: SchedulingAlgorithm,
    quantum: usize,
}

impl Station {
    pub fn new(
        name: &str,
        processing_time: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Product>>>,
        sender: Option<mpsc::Sender<Product>>,
        scheduling_algorithm: SchedulingAlgorithm,
        quantum: usize,
    ) -> Self {
        Station {
            name: name.to_string(),
            processing_time,
            receiver,
            sender,
            scheduling_algorithm,
            quantum,
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
        let mut rr_queue = Vec::new();
        
        loop {
            let mut new_product = None;
            
            {
                let receiver_lock = self.receiver.lock().unwrap();
                match receiver_lock.try_recv() {
                    Ok(p) => new_product = Some(p),
                    Err(mpsc::TryRecvError::Empty) => {},
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
            
            let mut processed = false;
            if let Some(ref mut product) = new_product {
                match self.scheduling_algorithm {
                    SchedulingAlgorithm::FCFS => {
                        println!("Estación {} procesando producto {} (FCFS)", self.name, product.id);
                        thread::sleep(Duration::from_millis(self.processing_time as u64));
                        println!("Estación {} terminó de procesar producto {} (FCFS)", self.name, product.id);
                        let product_to_send = product.clone();
                        self.sender.as_ref().unwrap().send(product_to_send).unwrap();
                        processed = true;
                        continue;
                    },
                    SchedulingAlgorithm::RoundRobin => {
                        if product.remaining_time.is_none() {
                            product.remaining_time = Some(self.processing_time);
                        }
                        println!("Añadiendo producto {} a la cola RR, tiempo restante: {}ms", 
                               product.id, product.remaining_time.unwrap_or(0));
                        let product_to_queue = product.clone();
                        rr_queue.push(product_to_queue);
                    },
                }
            }
            
            match self.scheduling_algorithm {
                SchedulingAlgorithm::FCFS => {
                    if !processed && new_product.is_none() {
                        let receiver_lock = self.receiver.lock().unwrap();
                        match receiver_lock.recv() {
                            Ok(p) => {
                                drop(receiver_lock);
                                println!("Estación {} procesando producto {} (FCFS - bloqueante)", self.name, p.id);
                                thread::sleep(Duration::from_millis(self.processing_time as u64));
                                println!("Estación {} terminó de procesar producto {} (FCFS - bloqueante)", self.name, p.id);
                                self.sender.as_ref().unwrap().send(p).unwrap();
                            }
                            Err(_) => break,
                        }
                    }
                },
                
                SchedulingAlgorithm::RoundRobin => {
                    if !rr_queue.is_empty() {
                        let mut product = rr_queue.remove(0);
                        
                        let remaining = product.remaining_time.unwrap_or(self.processing_time);
                        
                        let process_time = std::cmp::min(self.quantum, remaining);
                        
                        println!("Estación {} procesando producto {} por {}ms (tiempo restante: {}ms)", 
                                 self.name, product.id, process_time, remaining);
                        
                        thread::sleep(Duration::from_millis(process_time as u64));
                        
                        let new_remaining = remaining.saturating_sub(process_time);
                        product.remaining_time = Some(new_remaining);
                        
                        if new_remaining > 0 {
                            println!("Devolviendo producto {} a la cola, tiempo restante: {}ms", 
                                    product.id, new_remaining);
                            rr_queue.push(product);
                        } else {
                            println!("Estación {} terminó de procesar producto {}", self.name, product.id);
                            self.sender.as_ref().unwrap().send(product).unwrap();
                        }
                    } else if !processed && new_product.is_none() {
                        thread::sleep(Duration::from_millis(100));
                        
                        let receiver_lock = self.receiver.lock().unwrap();
                        match receiver_lock.recv_timeout(Duration::from_millis(500)) {
                            Ok(mut p) => {
                                drop(receiver_lock);
                                if p.remaining_time.is_none() {
                                    p.remaining_time = Some(self.processing_time);
                                }
                                println!("Recibido nuevo producto {} en cola RR (bloqueante)", p.id);
                                rr_queue.push(p);
                            },
                            Err(mpsc::RecvTimeoutError::Timeout) => {},
                            Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        }
                    }
                }
            }
        }
    }
}
