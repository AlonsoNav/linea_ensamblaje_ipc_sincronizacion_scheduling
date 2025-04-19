mod models;

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use std::sync::mpsc;
use rand::Rng;
use std::collections::VecDeque;
use std::io;
use crate::models::Station;
use crate::models::Product;
use crate::models::ProcessingStep;
use crate::models::SchedulingAlgorithm;

const PRODUCT_COUNT: usize = 10;
const STAGES: &[(&str, usize)] = &[
    ("Corte", 1000),
    ("Ensamblaje", 1500),
    ("Empaque", 800),
];
const DEFAULT_QUANTUM: usize = 500;

fn main() {
    let mut stations: Vec<Station> = Vec::new();
    let mut senders = Vec::new();
    let mut products = Vec::new();
    let mut rng = rand::thread_rng();
    
    // Select scheduling algorithm
    let scheduling_algorithm = select_scheduling_algorithm();
    
    // Get quantum for Round Robin if that algorithm is selected
    let quantum = match scheduling_algorithm {
        SchedulingAlgorithm::RoundRobin => {
            println!("Ingrese el quantum para Round Robin (milliseconds) o presione Enter para usar el valor predeterminado {}):", DEFAULT_QUANTUM);
            read_quantum()
        },
        _ => DEFAULT_QUANTUM, // Default value, not used for FCFS
    };

    // Main channel
    let (main_sender, main_receiver) = mpsc::channel();

    for &(name, processing_time) in STAGES.iter() {
        let (tx, rx) = mpsc::channel(); // Current channel

        let station = Station::new(
            name,
            processing_time,
            Arc::new(Mutex::new(rx)), 
            None, // Sender is set later 
            scheduling_algorithm,
            quantum,
        );
        
        if let Some(last_station) = stations.last_mut() {
            last_station.set_sender(tx.clone());
        }
        stations.push(station);
        senders.push(tx.clone()); 
    }

    // Set the main sender to the last station
    stations.last_mut().unwrap().set_sender(main_sender.clone()); 

    // Start all stations
    for station in stations {
        station.start();
    }

    let start_time = SystemTime::now(); 
    // Generate and send the products
    for i in 1..=PRODUCT_COUNT {
        let arrival_time = SystemTime::now() + Duration::from_millis(rng.gen_range(0..5000));
        let product = Product {
            id: i,
            arrival_time,
            processing_steps: vec![],
            remaining_time: None,
        };
        products.push(product);
    }

    // Sort products by arrival time
    products.sort_by_key(|p| p.arrival_time);

    // Use a queue to manage products
    let mut product_queue: VecDeque<Product> = VecDeque::from(products);

    // Send products to the first station based on arrival time
    while let Some(mut product) = product_queue.pop_front() {
        let now = SystemTime::now();
        if product.arrival_time <= now {
            product.arrival_time = now; // The real arrival time is now
            senders[0].send(product).unwrap();
        } else {
            // If the product hasn't arrived yet, reinsert it into the queue
            product_queue.push_front(product);
            std::thread::sleep(Duration::from_millis(100)); // Wait before retrying
        }
    }

    let mut processed_products = Vec::new();

    // Get the products processed by the main channel
    for _ in 1..=PRODUCT_COUNT {
        let processed_product = main_receiver.recv().unwrap();
        processed_products.push(processed_product.clone());
    }

    println!("\n--- Resumen Final ---");
    
    let mut total_wait_sum = 0.0;
    let mut total_turnaround_sum = 0.0;
    let mut orden_final: Vec<usize> = Vec::new();
    
    for product in &processed_products {
        if product.processing_steps.is_empty() {
            println!("\nProducto {} no tiene pasos registrados.", product.id);
            continue;
        }
    
        orden_final.push(product.id);
    
        let turnaround = product
            .processing_steps
            .last()
            .unwrap()
            .exit_time
            .unwrap()
            .duration_since(product.arrival_time)
            .unwrap()
            .as_secs_f64();
    
        println!("\nProducto {}", product.id);
        println!("Tiempo de llegada: {:.1}s", product.arrival_time.duration_since(start_time).unwrap().as_secs_f64());
    
        let mut total_wait = 0.0;
    
        for (i, step) in product.processing_steps.iter().enumerate() {
            let entry = step.entry_time.unwrap().duration_since(start_time).unwrap().as_secs_f64();
            let exit = step.exit_time.unwrap().duration_since(start_time).unwrap().as_secs_f64();
        
            let prev_exit = if i == 0 {
                product.arrival_time.duration_since(start_time).unwrap().as_secs_f64()
            } else {
                product.processing_steps[i - 1]
                    .exit_time
                    .unwrap()
                    .duration_since(start_time)
                    .unwrap()
                    .as_secs_f64()
            };
        
            total_wait += entry - prev_exit;
        
            println!("{}: entrada = {:.3}s, salida = {:.3}s", step.station_name, entry, exit);
        }
    
        println!("Tiempo de espera total: {:.3}s, Turnaround: {:.3}s", total_wait, turnaround);
    
        total_wait_sum += total_wait;
        total_turnaround_sum += turnaround;
    }
    
    let count = processed_products.len() as f64;
    
    let avg_wait = total_wait_sum / count;
    let avg_turnaround = total_turnaround_sum / count;
    
    println!("\nPromedio de espera: {:.9}s", avg_wait);
    println!("Promedio de turnaround: {:.9}s", avg_turnaround);
    
    println!("\nOrden final de procesamiento:");
    for id in orden_final {
        println!("Producto {}", id);
    }
    
}

// Function to let the user select the scheduling algorithm
fn select_scheduling_algorithm() -> SchedulingAlgorithm {
    println!("\nSeleccione el algoritmo de planificación:");
    println!("1. FCFS (First Come First Serve)");
    println!("2. Round Robin");
    
    loop {
        println!("Ingrese su elección (1 o 2):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        
        match input.trim() {
            "1" => return SchedulingAlgorithm::FCFS,
            "2" => return SchedulingAlgorithm::RoundRobin,
            _ => println!("Opción inválida, intente de nuevo."),
        }
    }
}

// Function to read quantum value for Round Robin
fn read_quantum() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return DEFAULT_QUANTUM;
    }
    
    match trimmed.parse::<usize>() {
        Ok(value) if value > 0 => value,
        _ => {
            println!("Valor inválido, usando el valor predeterminado: {}", DEFAULT_QUANTUM);
            DEFAULT_QUANTUM
        }
    }
}
