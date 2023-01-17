use tokio::runtime::Runtime;

pub trait RuntimeFactory {
    fn spawn(&self) -> Runtime;
}