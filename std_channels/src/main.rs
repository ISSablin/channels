use std::sync::mpsc;
use std::time::Instant;


pub fn main() {
    println!("Main started...");
    master_process();
    println!("Main stopped...");
}

pub fn master_process() {
    println!("Master process started...");
    let mut count = 0;
    let mut delays: Vec<u128> = vec![];
    let (client_master_tx, client_master_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel();
    
    std::thread::spawn(move || {
        client_process(client_master_tx);
    });

    while count < 1000 {
        let client_master_received = client_master_rx.try_recv();
        if !client_master_received.is_err() {
            delays.push(client_master_received.unwrap().elapsed().as_millis());
            count += 1;
        }
    }
    println!("Master process stopped...");
    delays.sort();
    println!("Maximum delay: {:?}ms Average delay: {:?}ms Median delay: {:?}ms", delays.iter().max().unwrap(), delays.iter().sum::<u128>() / delays.len() as u128, delays[delays.len()/2]);
}

pub fn client_process(client_master_tx: mpsc::Sender<Instant>) {
    println!("Client process started...");
    let mut count = 0;
    while count < 1000 {
        if count%100 == 0 {
            println!("Client cycle: {}/1000", count);
        }
        for n in 0..1000 {
            let x: f64 = 5324.3431 + n as f64;
            let _ = x.tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan();
        }
        // std::thread::sleep(std::time::Duration::from_millis(50));
        let _ = client_master_tx.send(Instant::now());
        count += 1;
    }
    println!("Client process stopped...");
}