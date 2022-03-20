use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Mutex::new(VecDeque::default()));
    (
        Sender {
            shared: Arc::clone(&shared), // 注意使用 Arc::clone 而不是 .clone
        },
        Receiver {
            shared: Arc::clone(&shared),
        },
    )
}

pub struct Sender<T> {
    shared: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutex
        let mut shared = self.shared.lock().unwrap();
        shared.push_back(value);
    }
}

impl<T> Clone for Sender<T> {
    // implement Clone istead of using #[derive(Clone)]
    // #[derive(Clone)] require that the T is Clone, but we don't want T to be Clone
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Mutex<VecDeque<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutexs
        // we want when there is no data on the queue, recv is blocked
        // when there is data on tge queue, return the first value
        // if there is no data, drop the lock, then rerun the loop.
        loop {
            let mut shared = self.shared.lock().unwrap();
            if let Some(value) = shared.pop_front() {
                return Some(value);
            } else {
                drop(shared); // when there is no sender, locks will be continuously aquired and droped.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (tx, rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn two_one() {
        let (tx, rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
        let tx1 = tx.clone();
        tx1.send(43);
        assert_eq!(rx.recv(), Some(43));
    }

    #[test]
    fn no_sender() {
        // expect
        let (tx, rx) = channel::<()>();
        drop(tx);
        let x = rx.recv();
        assert_eq!(x, None)
    }
}
