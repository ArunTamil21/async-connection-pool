use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tokio::sync::Semaphore;

use crate::connection::Connection;

#[derive(Debug, Clone)]
pub struct Pool {
    connections: Arc<Mutex<Vec<Connection>>>,
    limit: Arc<Semaphore>,
}

pub struct PoolGuard {
    connections: Arc<Mutex<Vec<Connection>>>,
    limit: Arc<Semaphore>,
    connection: Option<Connection>,
}

impl Pool {
    pub fn new(number: usize) -> Self {
        let mut connections = Vec::new();
        for i in 0..number {
            connections.push(Connection::new(i + 1));
        }
        Self {
            connections: Arc::new(Mutex::new(connections)),
            limit: Arc::new(Semaphore::new(number)),
        }
    }

    pub async fn get(&self) -> PoolGuard {
        let value = self.limit.acquire().await.expect("No permit available");
        value.forget();
        let mut guard = self.connections.lock().expect("Lock is poisoned");
        let connection = guard.pop().expect("Connection is empty");
        drop(guard);
        PoolGuard {
            connections: Arc::clone(&self.connections),
            limit: Arc::clone(&self.limit),
            connection: Some(connection),
        }
    }
}

impl Deref for PoolGuard {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        self.connection.as_ref().expect("Connection is empty")
    }
}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        let mut data = self.connections.lock().expect("Data is poisoned");
        data.push(self.connection.take().expect("No connection exists"));
        self.limit.add_permits(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, time::Duration};
    use tokio::time::timeout;

    #[tokio::test]
    async fn connection_capacity() {
        let pool = Pool::new(3);

        let _c1 = pool.get().await;
        let _c2 = pool.get().await;
        let _c3 = pool.get().await;

        let result = timeout(Duration::from_millis(10), pool.get()).await;
        assert!(result.is_err(), "get() should have blocked");
    }

    #[tokio::test]
    async fn connection_resume() {
        let pool = Pool::new(3);

        let _c1 = pool.get().await;
        let _c2 = pool.get().await;
        let _c3 = pool.get().await;

        let result = timeout(Duration::from_millis(10), pool.get()).await;
        assert!(result.is_err(), "get() should have blocked");

        drop(_c1);

        let result = timeout(Duration::from_millis(10), pool.get()).await;
        assert!(result.is_ok(), "get() should have release");
    }

    #[tokio::test]
    async fn connection_id_same() {
        let pool = Pool::new(1);

        let c1 = pool.get().await;

        let id1 = c1.id;

        drop(c1);

        let c2 = pool.get().await;

        let id2 = c2.id;

        assert_eq!(id1, id2);
    }
}
