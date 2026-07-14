use rand::random_range;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct Connection {
    pub id: usize,
}

impl Connection {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub async fn query(&self, _sql: &str) -> String {
        let delay = random_range(50..=200);
        sleep(Duration::from_millis(delay)).await;
        let id = self.id;

        format!("Result from connection id {id} in {delay} ms")
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_connection() {
        let value = Connection::new(1);
        assert_eq!(value.id, 1);
    }

    //async fn query_test() {
    //    let value = Connection::new(1);
    //    let data = value.query("data").await;
    //    assert_eq!(data, "Result from connection id 1".to_string());
    //}
}
