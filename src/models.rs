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
    pub station_name: String,
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
        let mut current_time = SystemTime::now();
        
        loop {
            // 1. Primero, recolectar todos los productos nuevos disponibles
            loop {
                let receiver_lock = self.receiver.lock().unwrap();
                match receiver_lock.try_recv() {
                    Ok(mut p) => {
                        drop(receiver_lock);
                        p.remaining_time = Some(self.processing_time);
                        println!("Estación {} recibió producto {} (tiempo restante: {}ms)", 
                               self.name, p.id, p.remaining_time.unwrap());
                        
                        // Insertar el producto en la cola basado en su tiempo de llegada
                        let insert_pos = rr_queue.iter()
                            .position(|existing: &Product| existing.arrival_time > p.arrival_time)
                            .unwrap_or(rr_queue.len());
                        rr_queue.insert(insert_pos, p);
                    },
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => return,
                }
            }

            if !rr_queue.is_empty() {
                let mut product = rr_queue.remove(0);
                let remaining = product.remaining_time.unwrap_or(self.processing_time);
                let process_time = std::cmp::min(self.quantum, remaining);
                
                println!("Estación {} procesando producto {} por {}ms (tiempo restante: {}ms)", 
                        self.name, product.id, process_time, remaining);
                
                let entry_time = SystemTime::now();
                thread::sleep(Duration::from_millis(process_time as u64));
                let exit_time = SystemTime::now();

                product.processing_steps.push(ProcessingStep {
                    station_name: self.name.clone(),
                    entry_time: Some(entry_time),
                    exit_time: Some(exit_time),
                });
                
                let new_remaining = remaining.saturating_sub(process_time);
                product.remaining_time = Some(new_remaining);
                
                if new_remaining > 0 {
                    println!("Devolviendo producto {} a la cola, tiempo restante: {}ms", 
                            product.id, new_remaining);
                    
                    // Reinsertar el producto al final de la cola
                    rr_queue.push(product);
                } else {
                    println!("Estación {} terminó de procesar producto {}", self.name, product.id);
                    product.remaining_time = None;
                    if let Some(sender) = &self.sender {
                        sender.send(product).unwrap();
                    }
                }
            } else {
                // Si no hay productos en la cola, esperar brevemente
                thread::sleep(Duration::from_millis(10));
                
                // Intentar recibir un nuevo producto de manera bloqueante pero con timeout
                let receiver_lock = self.receiver.lock().unwrap();
                match receiver_lock.recv_timeout(Duration::from_millis(200)) {
                    Ok(mut p) => {
                        drop(receiver_lock);
                        p.remaining_time = Some(self.processing_time);
                        println!("Estación {} recibió nuevo producto {} (bloqueante)", 
                               self.name, p.id);
                        rr_queue.push(p);
                    },
                    Err(mpsc::RecvTimeoutError::Timeout) => {},
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        }
    }
}

