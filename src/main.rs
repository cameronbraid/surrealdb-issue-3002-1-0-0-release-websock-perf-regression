use std::time::Instant;

use color_eyre::eyre::Result;
use surrealdb::{Surreal, engine::any::Any};
use tokio::task::JoinSet;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let surreal = surrealdb::engine::any::connect(format!("ws://localhost:12773"))
        .await
        .unwrap();

    surreal.use_ns("test").use_db("test").await.unwrap();

    let start = Instant::now();

    let mut join_set = JoinSet::new();
    for _ in 1..100_000 {
        join_set.spawn(run_task(surreal.clone()));
    }

    while let Some(data) = join_set.join_next().await {
        data?;
    }

    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}


async fn run_task(db: Surreal<Any>) {
    db.query("select * from [1]").await.unwrap().check().unwrap();
}