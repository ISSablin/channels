use tokio::sync::mpsc;
use std::time::Instant;

#[tokio::main]

pub async fn main() {
    println!("Main started...");

    // Start background tasks
    let s_t_1 = tokio::task::spawn_blocking(|| {sync_load_task(11)});
    let s_t_2 = tokio::task::spawn_blocking(|| {sync_load_task(12)});
    let s_t_3 = tokio::task::spawn_blocking(|| {sync_load_task(13)});
    let s_t_4 = tokio::task::spawn_blocking(|| {sync_load_task(14)});
    let s_t_5 = tokio::task::spawn_blocking(|| {sync_load_task(15)});

    // Start Master Process
    master_process().await;

    // Wait untill BG tasks are completed
    let _ = s_t_1.await;
    let _ = s_t_2.await;
    let _ = s_t_3.await;
    let _ = s_t_4.await;
    let _ = s_t_5.await;

    println!("Main stopped...");
}

pub async fn master_process() {
    println!("Master process started...");
    let mut count: usize = 0;
    let mut delays: Vec<u128> = vec![];

    let (master_handler_tx, master_handler_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(10);
    // tokio::task::spawn(async {
    //     handler_process(1, master_handler_rx).await;
    // });
    tokio::task::spawn_blocking(|| {
        let _ = handler_process(1, master_handler_rx);
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (client1_master_tx, mut client1_master_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(10);
    tokio::task::spawn(async {
        client_process(1, client1_master_tx).await;
    });
    let (client2_master_tx, mut client2_master_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(10);
    tokio::task::spawn(async {
        client_process(2, client2_master_tx).await;
    });

    while count < 2000 {
        let client1_master_received = client1_master_rx.try_recv();
        if !client1_master_received.is_err() {
            let instant_value = client1_master_received.unwrap();
            let _ = master_handler_tx.send(instant_value).await;
            delays.push(instant_value.elapsed().as_millis());
            count += 1;
        }
        let client2_master_received = client2_master_rx.try_recv();
        if !client2_master_received.is_err() {
            let instant_value = client2_master_received.unwrap();
            let _ = master_handler_tx.send(instant_value).await;
            delays.push(instant_value.elapsed().as_millis());
            count += 1;
        }
    }
    println!("Master process stopped...");
    delays.sort();
    println!("MASTER: Maximum delay: {:?}ms Average delay: {:?}ms Median delay: {:?}ms", delays.iter().max().unwrap(), delays.iter().sum::<u128>() / delays.len() as u128, delays[delays.len()/2]);
}

pub async fn client_process(id: usize, client_master_tx: mpsc::Sender<Instant>) {
    println!("Client process {} started...", id);
    let mut count = 0;
    while count < 1000 {
        if count%100 == 0 {
            println!("Client {} cycle: {}/1000", id, count);
        }
        system_load(2000).await;
//        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        let _ = client_master_tx.send(Instant::now()).await;
        count += 1;
    }
    println!("Client process {} stopped...", id);
}

pub fn handler_process(id: usize, mut master_client_rx: mpsc::Receiver<Instant>) {
    println!("Handler process {} started...", id);
    let mut count: usize = 0;
    let mut delays: Vec<u128> = vec![];
    while count < 2000 {
        let master_client_received = master_client_rx.try_recv();
        if !master_client_received.is_err() {
            delays.push(master_client_received.unwrap().elapsed().as_millis());
            count += 1;
        }
    }
    println!("Handler process {} stopped...", id);
    delays.sort();
    println!("HANDLER: Maximum delay: {:?}ms Average delay: {:?}ms Median delay: {:?}ms", delays.iter().max().unwrap(), delays.iter().sum::<u128>() / delays.len() as u128, delays[delays.len()/2]);
}

pub fn sync_load_task(id: usize) {
    println!("Load task {} started...", id);
    let mut count = 0;
    while count < 2000 {
        if count%100 == 0 {
            println!("Load task {} cycle: {}/2000", id, count);
        }
        sync_system_load(1000);
        count += 1;
    }
    println!("Load task {} stopped...", id);

}

pub async fn load_task(id: usize) {
    println!("Load task {} started...", id);
    let mut count = 0;
    while count < 2000 {
        if count%100 == 0 {
            println!("Load task {} cycle: {}/2000", id, count);
        }
        system_load(1000).await;
        count += 1;
    }
    println!("Load task {} stopped...", id);

}

pub fn sync_system_load(number_of_cycles: usize) {
    for n in 0..number_of_cycles {
        let x: f64 = 5324.3431 + n as f64;
        let _ = x.tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan();
    }
}

pub async fn system_load(number_of_cycles: usize) {
    for n in 0..number_of_cycles {
        let x: f64 = 5324.3431 + n as f64;
        let _ = x.tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan();
    }
}