use tokio::sync::mpsc;
use std::time::Instant;


#[tokio::main]

pub async fn main() {
    println!("Main started...");
    master_process().await;
    println!("Main stopped...");
}

pub async fn master_process() {
    println!("Master process started...");
    let mut count = 0;
    let mut delays: Vec<u128> = vec![];
    let (client_master_tx, mut client_master_rx): (mpsc::UnboundedSender<Instant>, mpsc::UnboundedReceiver<Instant>) = mpsc::unbounded_channel();
    
    tokio::task::spawn(async {
        client_process(client_master_tx).await;
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

pub async fn client_process(client_master_tx: mpsc::UnboundedSender<Instant>) {
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
        let _ = client_master_tx.send(Instant::now());
        count += 1;
    }
    println!("Client process stopped...");
}