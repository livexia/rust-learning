use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        counter: 1,
    };
    let shared = Arc::new(Shared {
        inner: Mutex::new(inner),
        availability: Condvar::new(),
    });
    (
        Sender {
            shared: Arc::clone(&shared), // 注意使用 Arc::clone 而不是 .clone
        },
        Receiver {
            shared: Arc::clone(&shared),
            buffer: VecDeque::default(), // add buffer for recv optimization
        },
    )
}

pub struct Inner<T> {
    // counter need be inside the mutex
    // so there is another warpper for the queue and counter
    queue: VecDeque<T>,
    counter: usize,
}

pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    availability: Condvar, // Condvar cna not be in Muttex, so need another wrapper for the queue
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutex
        let mut shared = self.shared.inner.lock().unwrap();
        shared.queue.push_back(value);

        self.shared.availability.notify_one();
        // after send notify the recv there is data on the queue, recvier can be wake up.
    }
}

impl<T> Clone for Sender<T> {
    // implement Clone istead of using #[derive(Clone)]
    // #[derive(Clone)] require that the T is Clone, but we don't want T to be Clone
    fn clone(&self) -> Self {
        // when clone sender need to incremental the count by one
        // now we need acquire the lock to modify the counter
        let mut inner = self.shared.inner.lock().unwrap();
        inner.counter += 1;
        drop(inner); // drop inner
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    // now we drop the sender, we need to subtract the counter by one
    fn drop(&mut self) {
        // now we also need acquire the lock to modify the counter
        let mut inner = self.shared.inner.lock().unwrap();
        inner.counter -= 1;
        // decremental counter
        let was_last = inner.counter == 0;
        drop(inner);
        if was_last {
            // when there is no sender, notify receiver should be wake to return None
            self.shared.availability.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
    // add buffer to receiver,
    // because there is only one receiver, so it is ok to put buffer outside the Mutex
    // when there is data one the buffer, just pop from the buffer, no need to acquire the lock
    // when there is nothing on the buffer, acquire the lock, when there is data in the queue,
    // pop the first one, and swap the buffer and queue.
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        // we want when there is no data on the queue, recv is blocked
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutexs
        // because of the buffer, we need &mut self, to quick swap the buffer and the queue.
        if let Some(t) = self.buffer.pop_front() {
            // when there is data one the buffer, just pop from the buffer, no need to acquire the lock
            return Some(t);
        }

        // when there is data on tge queue, return the first value
        // if there is no data, drop the lock, then rerun the loop.
        let mut shared = self.shared.inner.lock().unwrap();
        loop {
            match shared.queue.pop_front() {
                Some(t) => {
                    ::std::mem::swap(&mut self.buffer, &mut shared.queue);
                    return Some(t);
                }
                None if shared.counter == 0 => return None,
                // when receiver wake up, and there is no sender, recv return None
                None => {
                    // drop(shared);
                    // when there is no data, locks will be continuously aquired and dropped.
                    // We need a way for the receiver to sleep when there is no data,
                    // and when there is more date on the queue, we need to wake up the receiver.
                    // we use Condvar see https://doc.rust-lang.org/std/sync/struct.Condvar.html

                    shared = self.shared.availability.wait(shared).unwrap();
                    // unwrap to ignore possible thread poison
                    // when there is no sender, this will be hang,
                    // so we need a counter to keep track of the number of senders.
                    // counter should be inside the mutex, or anohter atomic counter outside the mutex
                    // need another wrapper for the sender's counter
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn two_one() {
        let (tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
        let tx1 = tx.clone();
        tx1.send(43);
        assert_eq!(rx.recv(), Some(43));
    }

    #[test]
    fn no_sender() {
        // expect not hang
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        let x = rx.recv();
        assert_eq!(x, None)
    }

    #[test]
    fn across_two_thread() {
        // this will hang because tx not drop
        // always will remain atleast one sender
        use std::thread;
        let (tx, mut rx) = channel();
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        thread::spawn(move || tx1.send(42));
        thread::spawn(move || tx2.send(43));
        drop(tx); // without this will hang

        let x1 = rx.recv();
        let x2 = rx.recv();
        let x3 = rx.recv();
        assert_eq!(x1, Some(42));
        assert_eq!(x2, Some(43));
        assert_eq!(x3, None);
    }
}
