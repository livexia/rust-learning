use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//   - Mutex + Condvar + VecDeque
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of atomic VecDeque<T>
//  - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//  - Oneshot channels: Any capacity. In practice, only one call to send().

pub fn sync_channel<T>(bound: usize) -> (AsyncSender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::with_capacity(bound + 1),
        counter: 1,
        bound,
    };
    let shared = Arc::new(Shared {
        inner: Mutex::new(inner),
        recv_availability: Condvar::new(),
        send_availability: Condvar::new(),
    });
    (
        AsyncSender {
            shared: Arc::clone(&shared), // 注意使用 Arc::clone 而不是 .clone
        },
        Receiver {
            shared: Arc::clone(&shared),
        },
    )
}

pub struct Inner<T> {
    // counter need be inside the mutex
    // so there is another warpper for the queue and counter
    queue: VecDeque<T>,
    counter: usize,
    bound: usize,
}

pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    recv_availability: Condvar, // Condvar cna not be in Muttex, so need another wrapper for the queue
    send_availability: Condvar, // Condvar cna not be in Muttex, so need another wrapper for the queue
}

pub struct AsyncSender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> AsyncSender<T> {
    pub fn send(&self, value: T) {
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutex
        let mut shared = self.shared.inner.lock().unwrap();
        loop {
            // when bound is 0 and there is no data on the queue
            // allow once push_back, after push notify the receiver and block the sender
            if shared.bound == 0 && shared.queue.len() == 0 {
                shared.queue.push_back(value);
                self.shared.recv_availability.notify_one();
                let _ = self.shared.send_availability.wait(shared).unwrap();
                return;
            };
            if shared.queue.len() < shared.bound {
                shared.queue.push_back(value);

                self.shared.recv_availability.notify_one();
                return;
                // after send notify the recv there is data on the queue, recvier can be wake up.
            } else {
                shared = self.shared.send_availability.wait(shared).unwrap();
                // when there is no more room for the sender,
                // block send, wait until some receiver recv the data
            }
        }
    }
}

impl<T> Clone for AsyncSender<T> {
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

impl<T> Drop for AsyncSender<T> {
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
            self.shared.recv_availability.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    // add buffer to receiver,
    // because there is only one receiver, so it is ok to put buffer outside the Mutex
    // when there is data one the buffer, just pop from the buffer, no need to acquire the lock
    // when there is nothing on the buffer, acquire the lock, when there is data in the queue,
    // pop the first one, and swap the buffer and queue.
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        // we want when there is no data on the queue, recv is blocked
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutexs

        // when there is data on tge queue, return the first value
        // if there is no data, drop the lock, then rerun the loop.
        let mut shared = self.shared.inner.lock().unwrap();
        loop {
            match shared.queue.pop_front() {
                Some(t) => {
                    // eprintln!("inside lock");
                    // notify_all sender when the data were taken out
                    self.shared.send_availability.notify_all();
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

                    shared = self.shared.recv_availability.wait(shared).unwrap();
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
        let (tx, rx) = sync_channel(1);
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn two_one() {
        let (tx, rx) = sync_channel(2);
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
        let tx1 = tx.clone();
        tx1.send(43);
        assert_eq!(rx.recv(), Some(43));
    }

    #[test]
    fn no_sender() {
        // expect not hang
        let (tx, rx) = sync_channel::<()>(1);
        drop(tx);
        let x = rx.recv();
        assert_eq!(x, None)
    }

    #[test]
    fn across_two_thread() {
        // this will hang because tx not drop
        // always will remain atleast one sender
        use std::thread;
        let (tx, rx) = sync_channel(1);
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        thread::spawn(move || tx1.send(42));
        thread::spawn(move || tx2.send(43));
        drop(tx); // without this will hang

        let x1 = rx.recv();
        let x2 = rx.recv();
        let x3 = rx.recv();
        assert_eq!(85, x1.unwrap() + x2.unwrap());
        assert_eq!(x3, None);
    }

    #[test]
    fn sync_channel_rendezvous_test() {
        // this will hang because sync_channel cannot handle size 0 channel
        // see https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html#method.send
        use std::thread;

        // Create a rendezvous sync_channel with buffer size 0
        let (sync_sender, receiver) = sync_channel(0);

        thread::spawn(move || {
            println!("sending message...");
            sync_sender.send(1);
            // Thread is now blocked until the message is received

            println!("...message received!");
        });

        let msg = receiver.recv().unwrap();
        println!("msg: {}", msg);
        assert_eq!(1, msg);
    }
}
