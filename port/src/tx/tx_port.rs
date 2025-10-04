use std::sync::Arc;

pub struct TodoTx {}

pub trait TxPort<D> {
    fn get(&self) -> D;
    fn unit_of_work(uof: fn(tx: Arc<Self>));
}
