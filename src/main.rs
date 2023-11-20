use std::time::Instant;

use color_eyre::eyre::Result;
use fake::{Dummy, Faker};
use serde::{Serialize, Deserialize};
use surrealdb::{engine::any::Any, Surreal};
use tokio::task::JoinSet;
use fake::Fake;


#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let surreal = surrealdb::engine::any::connect(format!("ws://localhost:8002"))
        .await
        .unwrap();

    surreal.use_ns("test").use_db("test").await.unwrap();

    init_task(surreal.clone()).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let start = Instant::now();

    let mut join_set = JoinSet::new();
    for _ in 1..1000 {
        join_set.spawn(run_task(surreal.clone()));
    }

    while let Some(data) = join_set.join_next().await {
        data?;
    }

    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}

#[derive(Debug, Dummy, Serialize, Deserialize)]
pub struct Foo {
    pk: usize,
    str_1: String,
    str_2: String,
    str_3: String,
    paid: bool,
}

#[derive(Debug, Dummy, Serialize, Deserialize)]
pub struct Bar {
    pk: usize,
    str_1: String,
    str_2: String,
    str_3: String,
    paid: bool,
}

async fn run_task(db: Surreal<Any>) {
    let i = Faker.fake::<usize>() % 100;
    let i2 = Faker.fake::<usize>() % 100;
    db.query("
        select (select ->has->bar from $parent) as bar, (select * from $parent) as foo from foo limit 100 start at $offset;
        
        select (select <-has<-foo from $parent) as foo, (select * from $parent) as bar from bar limit 100 start at $offset2;
        
    ")
    .bind(("offset", i))
    .bind(("offset2", i2))
        .await
        .unwrap()
        .check()
        .unwrap();
}
async fn init_task(db: Surreal<Any>) {
    db.query("delete foo; delete bar")
        .await
        .unwrap()
        .check()
        .unwrap();
    let mut join_set = JoinSet::new();

    for id in 1..10_000 {
        join_set.spawn({
            let db = db.clone();
            async move {
                let mut foo = Faker.fake::<Foo>();
                foo.pk = id;
                let mut bar = Faker.fake::<Bar>();
                bar.pk = id;

                db.query(
                    "
                
                    create foo:[$foo.pk] content $foo return none;

                    create bar:[$bar.pk] content $bar return none;

                    relate foo:[$foo.pk]->has->bar:[$bar.pk] return none;
                ",
                )
                .bind(("foo", foo))
                .bind(("bar", bar))
                .await
                .unwrap()
                .check()
                .unwrap();
            }
        });
    }

    while let Some(data) = join_set.join_next().await {
        data.unwrap();
    }
}
