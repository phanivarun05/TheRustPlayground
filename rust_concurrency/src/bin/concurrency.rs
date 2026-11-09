use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn fetch_worker(id: i32) {
    println!("Worker {} started", id);
    sleep(Duration::from_secs(1)).await;
    println!("Worker {} done!", id);
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    for i in 1..=3 {
        set.spawn(fetch_worker(i));
    }

    while let Some(result) = set.join_next().await {
        if let Err(e) = result {
            eprintln!("A task failed or panicked: {:?}", e);
        }
    }

    println!("All workers finished successfully.");
}