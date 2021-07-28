use unstuck::{api, yolo};

#[tokio::main]
async fn main() -> Result<(), api::Error> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        None => println!("usage: unstuck <namespace>"),
        Some(ns) => yolo::brute_force_fix(&ns).await?,
    }

    Ok(())
}
