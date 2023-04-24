use std::thread;
use std::sync::mpsc::channel;
use tokio::runtime::Handle;

uniffi::include_scaffolding!("bug_finder");

#[uniffi::export(async_runtime = "tokio")]
pub async fn add_async(left: u8, right: u8) -> u8 {
    let (to_thread_tx, to_thread_rx) = channel();
    let (from_thread_tx, from_thread_rx) = channel();
    let handle = Handle::current();

    println!("spawning thread...");
    thread::spawn( move || {
        println!("thread: spawned");
        handle.block_on(async {
            println!("timer waiting...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("channel waiting...");
            let _ = to_thread_rx.recv().unwrap();
            from_thread_tx.send(0).unwrap();
        });
    });
    to_thread_tx.send(0).unwrap();
    let _ = from_thread_rx.recv().unwrap();
    left + right
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn add_async_normal(left: u8, right: u8) -> u8 {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn async_test() {
        println!("waiting for add...");
        let result = add_async(2, 2).await;
        assert_eq!(result, 4);
    }
}
