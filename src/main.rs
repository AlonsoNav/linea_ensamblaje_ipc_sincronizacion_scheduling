mod models;

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, Duration};
use std::sync::mpsc;
use rand::Rng;
use std::collections::VecDeque;
use crate::models::Station;
use crate::models::Product;

const PRODUCT_COUNT: usize = 10;
const STAGES: &[(&str, usize)] = &[
    ("Corte", 1000),
    ("Ensamblaje", 1500),
    ("Empaque", 800),
];

fn main() {
    let mut stations: Vec<Station> = Vec::new();
    let mut senders = Vec::new();
    let mut products = Vec::new();
    let mut rng = rand::rng();

    // Main channel
    let (main_sender, main_receiver) = mpsc::channel();

    for &(name, processing_time) in STAGES.iter() {
        let (tx, rx) = mpsc::channel(); // Current channel

        let station = Station::new(
            name,
            processing_time,
            Arc::new(Mutex::new(rx)), 
            None, // Sender is set later 
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

    // Generate and send the products
    for i in 1..=PRODUCT_COUNT {
        let arrival_time = SystemTime::now() + Duration::from_millis(rng.random_range(0..5000)); // Random arrival time between 0 and 5 seconds 
        let product = Product {
            id: i,
            arrival_time,
            processing_steps: vec![],
        };
        products.push(product);
    }

    // Sort products by arrival time
    products.sort_by_key(|p| p.arrival_time);

    // Use a queue to manage products
    let mut product_queue: VecDeque<Product> = VecDeque::from(products);

    // Send products to the first station based on arrival time
    while let Some(product) = product_queue.pop_front() {
        let now = SystemTime::now();
        if product.arrival_time <= now {
            println!("Enviando producto {} a la primera estación.", product.id);
            senders[0].send(product).unwrap();
        } else {
            // If the product hasn't arrived yet, reinsert it into the queue
            product_queue.push_front(product);
            std::thread::sleep(Duration::from_millis(100)); // Wait before retrying
        }
    }

    // Get the products processed by the main channel
    for _ in 1..=PRODUCT_COUNT {
        let processed_product = main_receiver.recv().unwrap();
        println!("Producto procesado: {}", processed_product.id);
    }
}
