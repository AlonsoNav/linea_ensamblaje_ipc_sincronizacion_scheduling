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
        let mut queue = Vec::new();
        let mut current_index = 0;
        
        loop {
            // Verificar si hay nuevos productos y añadirlos a la cola
            self.check_for_new_products(&mut queue);
            
            match self.scheduling_algorithm {
                SchedulingAlgorithm::FCFS => {
                    if !queue.is_empty() {
                        let mut product = queue.remove(0); // Tomar el primero (FCFS)
                        
                        let entry_time = SystemTime::now();
                        thread::sleep(Duration::from_millis(self.processing_time as u64));
                        let exit_time = SystemTime::now();
                        
                        product.processing_steps.push(ProcessingStep {
                            station_name: self.name.clone(),
                            entry_time: Some(entry_time),
                            exit_time: Some(exit_time),
                        });
                        
                        if let Some(sender) = &self.sender {
                            sender.send(product).unwrap();
                        }
                    } else {
                        // Si no hay productos en la cola, hacer un recv bloqueante
                        let receiver_lock = self.receiver.lock().unwrap();
                        match receiver_lock.recv() {
                            Ok(mut p) => {
                                drop(receiver_lock);
                                let entry_time = SystemTime::now();
                                thread::sleep(Duration::from_millis(self.processing_time as u64));
                                let exit_time = SystemTime::now();
                                
                                p.processing_steps.push(ProcessingStep {
                                    station_name: self.name.clone(),
                                    entry_time: Some(entry_time),
                                    exit_time: Some(exit_time),
                                });
                                
                                if let Some(sender) = &self.sender {
                                    sender.send(p).unwrap();
                                }
                            }
                            Err(_) => break,
                        }
                    }
                },
                
                SchedulingAlgorithm::RoundRobin => {
                    if !queue.is_empty() {
                        // Procesar el producto en la posición actual del iterador
                        let mut product = queue.remove(current_index);
                        
                        // Inicializar remaining_time si es necesario
                        if product.remaining_time.is_none() {
                            product.remaining_time = Some(self.processing_time);
                        }
                        
                        let remaining = product.remaining_time.unwrap();
                        let process_time = std::cmp::min(self.quantum, remaining);
                        
                        // Procesar el producto por el tiempo del quantum
                        let entry_time = SystemTime::now();
                        thread::sleep(Duration::from_millis(process_time as u64));
                        let exit_time = SystemTime::now();
                        
                        // Registrar el paso de procesamiento
                        product.processing_steps.push(ProcessingStep {
                            station_name: self.name.clone(),
                            entry_time: Some(entry_time),
                            exit_time: Some(exit_time),
                        });
                        
                        // Actualizar el tiempo restante
                        let new_remaining = remaining.saturating_sub(process_time);
                        product.remaining_time = Some(new_remaining);
                        
                        // Comprobar si hay nuevos productos después del quantum
                        self.check_for_new_products(&mut queue);
                        
                        // Decidir qué hacer con el producto actual
                        if new_remaining == 0 {
                            // Si el producto ha terminado, enviarlo a la siguiente estación
                            product.remaining_time = None; // Limpiar el tiempo restante
                            if let Some(sender) = &self.sender {
                                sender.send(product).unwrap();
                            }
                            // No reinsertamos el producto en la cola
                            
                            // Ajustar el índice si es necesario
                            if !queue.is_empty() {
                                current_index %= queue.len();
                            } else {
                                current_index = 0;
                            }
                        } else {
                            // Si el producto aún no ha terminado, devolverlo a la cola
                            queue.insert(current_index, product);
                            
                            // Avanzar al siguiente producto en la cola
                            current_index = (current_index + 1) % queue.len().max(1);
                        }
                    } else {
                        // Si la cola está vacía, esperar un poco
                        thread::sleep(Duration::from_millis(100));
                        
                        // Intentar recibir un nuevo producto (bloqueante con timeout)
                        let receiver_lock = self.receiver.lock().unwrap();
                        match receiver_lock.recv_timeout(Duration::from_millis(500)) {
                            Ok(p) => {
                                drop(receiver_lock);
                                queue.push(p);
                            }
                            Err(mpsc::RecvTimeoutError::Timeout) => {}
                            Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        }
                    }
                }
            }
        }
    }
    
    // Método para revisar si hay nuevos productos y añadirlos a la cola
    fn check_for_new_products(&self, rr_queue: &mut Vec<Product>) {
        let receiver_lock = self.receiver.lock().unwrap();
        
        // Intentar recibir todos los productos disponibles
        loop {
            match receiver_lock.try_recv() {
                Ok(mut p) => {
                    // Inicializar remaining_time si es necesario
                    if p.remaining_time.is_none() {
                        p.remaining_time = Some(self.processing_time);
                    }
                    
                    rr_queue.push(p);
                },
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }
    }
}
