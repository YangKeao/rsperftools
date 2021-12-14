// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.
#![feature(bench_black_box)]

use pprof;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[tokio::main]
async fn main() {
    let guard = pprof::pyroscope::PyroscopeAgent::new(
        "http://localhost:4040".to_owned(),
        99,
        "fibonacci".to_owned(),
        Some(
            [
                ("TagA".to_owned(), "ValueA".to_owned()),
                ("TagB".to_owned(), "ValueB".to_owned()),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
    )
    .await;

    for s in &[1, 10, 40, 50] {
        let result = fibonacci(criterion::black_box(*s));
        println!("fibonacci({}) -> {}", *s, result);
    }

    guard.stop().await.unwrap();

    return;
}