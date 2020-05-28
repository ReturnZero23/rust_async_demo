use async_std::{fs::File, io, prelude::*, task};
use std::thread;
use std::time::Duration;


async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    let reader_task = task::spawn(async {
        println!("start read");
        thread::sleep(Duration::from_millis(10000));
        let result = read_file("data.csv").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e)
        }
    });
    thread::sleep(Duration::from_millis(10));
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}