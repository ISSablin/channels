use tokio::sync::mpsc;
use std::time::Instant;
use futures::FutureExt;

#[tokio::main]

pub async fn main() {
    println!("Main started...");

    // Start background tasks
    // let s_t_1 = tokio::task::spawn_blocking(|| {sync_load_task(11)});
    // let s_t_2 = tokio::task::spawn_blocking(|| {sync_load_task(12)});
    // let s_t_3 = tokio::task::spawn_blocking(|| {sync_load_task(13)});
    // let s_t_4 = tokio::task::spawn_blocking(|| {sync_load_task(14)});
    // let s_t_5 = tokio::task::spawn_blocking(|| {sync_load_task(15)});

    tokio::task::spawn(async {
        load_task(11).await;
    });
    tokio::task::spawn(async {
        load_task(12).await;
    });
    tokio::task::spawn(async {
        load_task(13).await;
    });
    tokio::task::spawn(async {
        load_task(14).await;
    });
    tokio::task::spawn(async {
        load_task(15).await;
    });

    // Start Master Process
    master_process().await;

    // Wait untill BG tasks are completed
    // let _ = s_t_1.await;
    // let _ = s_t_2.await;
    // let _ = s_t_3.await;
    // let _ = s_t_4.await;
    // let _ = s_t_5.await;

    println!("Main stopped...");
}

use std::task::{Context, Waker, RawWaker, RawWakerVTable};


fn wake(_ptr: *const ()) {}

fn wake_by_ref(_ptr: *const ()) {}

fn drop(_ptr: *const ()) {}

fn clone(ptr: *const ()) -> RawWaker {


    RawWaker::new(ptr, &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone,
    wake,
    wake_by_ref,
    drop,
);

pub async fn master_process() {
    println!("Master process started...");
    let mut count: usize = 0;
    let mut delays: Vec<u128> = vec![];

    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe {Waker::from_raw(raw)};
    let mut _context = Context::from_waker(&waker);
    let raw2 = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker2 = unsafe {Waker::from_raw(raw2)};
    let mut _context2 = Context::from_waker(&waker2);
    let raw3 = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker3 = unsafe {Waker::from_raw(raw3)};
//    let mut _context3 = Context::from_waker(&waker3);

    // Handler channel

    let (master_handler_tx, master_handler_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(1);
    tokio::task::spawn(async {
        handler_process(1, master_handler_rx, waker3).await;
    });
    // tokio::task::spawn_blocking(|| {
    //     let _ = handler_process(1, master_handler_rx);
    // });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (client1_master_tx, mut client1_master_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(32);
    tokio::task::spawn(async {
        client_process(1, client1_master_tx).await;
    });

    // This is aditional channel if needed

    let (client2_master_tx, mut client2_master_rx): (mpsc::Sender<Instant>, mpsc::Receiver<Instant>) = mpsc::channel(1);
    tokio::task::spawn(async {
        client_process(2, client2_master_tx).await;
    });


    while count < 2000 {

        // First client

        // OPTION 1
        // This works and works well, but it blocks the loop. That's the reason we can't use recv().await

        // let client1_master_received = client1_master_rx.recv().await;
        // if client1_master_received != None {
        //     let instant_value = client1_master_received.unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     let _ = master_handler_tx.send(instant_value).await;
        //     //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
        //     }
        // }

        //OPTION 2
        // This works

        // let client1_master_received = client1_master_rx.recv().now_or_never();
        // if client1_master_received != None && client1_master_received.unwrap() != None {
        //     let instant_value = client1_master_received.unwrap().unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     let _ = master_handler_tx.send(instant_value).await;
        //     tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
        //     //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
        //     }    
        // }

        // OPTION 3
        // This works

        let client1_master_received = client1_master_rx.poll_recv(&mut _context);
        if client1_master_received.is_ready() {
           if let std::task::Poll::Ready(instant_value) = client1_master_received {
                if !instant_value.is_none() {
                    delays.push(instant_value.unwrap().elapsed().as_millis());
                    let _ = master_handler_tx.send(instant_value.unwrap()).await;
                    //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value.unwrap());
                    count += 1;
                    if count%100 == 0 {
                        println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
                    }
                } else {
                    println!("MASTER: None received... Count: {}", count);
                    let _ = master_handler_tx.send(Instant::now()).await;
                    count += 1;
                    //break
                }
           }
        }

        // Second client if needed

        // OPTION 1
        // This works and works well, but it blocks the loop. That's the reason we can't use recv().await

        // let client2_master_received = client2_master_rx.recv().await;
        // if client2_master_received != None {
        //     let instant_value = client2_master_received.unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     let _ = master_handler_tx.send(instant_value).await;
        //     //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
        //     }
        // }

        //OPTION 2
        // This works

        // let client2_master_received = client2_master_rx.recv().now_or_never();
        // if client2_master_received != None && client2_master_received.unwrap() != None {
        //     let instant_value = client2_master_received.unwrap().unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     let _ = master_handler_tx.send(instant_value).await;
        //     tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
        //     //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
        //     }    
        // }

        // OPTION 3
        // This works

        let client2_master_received = client2_master_rx.poll_recv(&mut _context2);
        if client2_master_received.is_ready() {
           if let std::task::Poll::Ready(instant_value) = client2_master_received {
                if !instant_value.is_none() {
                    delays.push(instant_value.unwrap().elapsed().as_millis());
                    let _ = master_handler_tx.send(instant_value.unwrap()).await;
                    //println!("RECEIVER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value.unwrap());
                    count += 1;
                    if count%100 == 0 {
                        println!("Master process cycle: {}/2000 received: {}/2000", count, delays.len());
                    }
                } else {
                    println!("MASTER: None received... Count: {}", count);
                    let _ = master_handler_tx.send(Instant::now()).await;
                    count += 1;
                    //break
                }
           }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    println!("Master process stopped...");
    delays.sort();
    println!("MASTER: Maximum delay: {:?}ms Average delay: {:?}ms Median delay: {:?}ms", delays.iter().max().unwrap(), delays.iter().sum::<u128>() / delays.len() as u128, delays[delays.len()/2]);
}

pub async fn client_process(id: usize, client_master_tx: mpsc::Sender<Instant>) {
    println!("Client process {} started...", id);
    let mut count: usize = 0;
    while count < 1000 {
        if count%100 == 0 {
            println!("Client {} cycle: {}/1000", id, count);
        }
        //tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        system_load(2000).await;
        let _res = client_master_tx.send(Instant::now()).await;
        //println!("SENDER: Count {} Channel is closed: {:?}", count, client_master_tx.is_closed());
        count += 1;
    }
    println!("Client process {} stopped...", id);
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

pub async fn handler_process(id: usize, mut master_handler_rx: mpsc::Receiver<Instant>, wk: Waker) {
    println!("Handler process {} started...", id);
    let mut count: usize = 0;
    let mut delays: Vec<u128> = vec![];

    let mut _context3 = Context::from_waker(&wk);

    while count < 2000 {

        // OPTION 1
        // This works and works well, but it blocks the loop. That's the reason we can't use recv().await

        // let master_handler_received = master_handler_rx.recv().await;
        // if master_handler_received != None {
        //     let instant_value = master_handler_received.unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     //println!("HANDLER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Handler process cycle: {}/2000, received: {}/2000", count, delays.len());
        //     }
        // }

        //OPTION 2
        // This works

        // let master_handler_received = master_handler_rx.recv().now_or_never();
        // if master_handler_received != None && master_handler_received.unwrap() != None {
        //     let instant_value = master_handler_received.unwrap().unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
        //     //println!("HANDLER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        //     if count%100 == 0 {
        //         println!("Handler process cycle: {}/2000, received: {}/2000", count, delays.len());
        //     }    
        // }

        // OPTION 3
        // This works

        let master_handler_received = master_handler_rx.poll_recv(&mut _context3);
        if master_handler_received.is_ready() {
           if let std::task::Poll::Ready(instant_value) = master_handler_received {
                if !instant_value.is_none() {
                    delays.push(instant_value.unwrap().elapsed().as_millis());
                    //println!("HANDLER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value.unwrap());
                    count += 1;
                    if count%100 == 0 {
                        println!("Handler process cycle: {}/2000, received: {}/2000", count, delays.len());
                    }
                } else {
                    println!("HANDLER: None received...");
                    break
                }
           }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;

        // Blocking channel
        // if count%100 == 0 {
        //     println!("Handler process cycle: {}/1000", count);
        // }
        // let master_handler_received = master_handler_rx.recv().await;
        // if master_handler_received != None {
        //     let instant_value = master_handler_received.unwrap();
        //     delays.push(instant_value.elapsed().as_millis());
        //     //println!("HANDLER: Count: {} Delay: {} Received: {:?}", count, delays[count],instant_value);
        //     count += 1;
        // }

        // Implementation for tokio 0.3.6
        // let master_handler_received = master_handler_rx.try_recv();
        // if !master_handler_received.is_err() {
        //     delays.push(master_handler_received.unwrap().elapsed().as_millis());
        //     count += 1;
        // }
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
        if n%100 == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
        }
        let x: f64 = 5324.3431 + n as f64;
        let _ = x.tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan().tan().atan();
    }
}