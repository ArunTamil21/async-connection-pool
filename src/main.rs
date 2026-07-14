//  6. Write tests:
//     - Pool returns connections up to its capacity without blocking
//     - A task blocks when the pool is exhausted, and proceeds once a
//       connection is returned
//     - Connections are reused (same IDs appear multiple times)
//

use std::sync::Arc;

use connection_pool::pool;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut store = JoinSet::new();
    let connection_pool = Arc::new(pool::Pool::new(3));

    for i in 0..10 {
        let pool_clone = Arc::clone(&connection_pool);
        store.spawn(async move {
            let connection = pool_clone.get().await;
            let data = connection.query(&"hello").await;
            println!("Task {i} requests Query");
            println!("{data}");
        });
    }

    store.join_all().await;
}
