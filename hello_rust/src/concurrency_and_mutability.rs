use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

fn threading() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread");
    });
    handle.join().unwrap();
}

/// ```moving data``` into the scope.
/// Just like any other scope, a spawn thread take variable requires either ownership of a value or
/// at least a borrowed reference in order to work with data
fn threading_move() {
    let x = 10;
    let s = String::from("hello");
    let handle = thread::spawn(|| {
        // this statement will going to fail, the reason is simple:
        // the compiler cannot determine the lifetimes of each of the scopes(will x still be there when the thread needs it?)
        // and compiler will indicate that adding the ```move``` keyword will sove the issue!
        // This keyword lets a thread pass ownership to a different thread, it ```move``` the memory data
        // (how about use Rc<RefCell<T>> in thread? or it's Metux?)
        // ```println!("Hello from a thread, the number is {}", x);```
    });
    let handle = thread::spawn(move || {
        /// However, for passing multiple messages into a thread or implementing ac ```actor model```,
        /// the Rust standard library offers ```channels```, channels are single-consumer, multiple producer queues that
        /// let the caller send messages from multiple threads
        println!("Hello from a thread, the number is {}", x);
        println!("Hello from a thread, the string is {}", s);
    });
    // this ok, x is Copy
    println!("x after move into thread: {}", x);
    // error: value borrowed here after move
    // println!("s after move into thread: {}", s);
    handle.join().unwrap();
}

/// With channels, a multithreaded application can move data between threads without the need
/// for manual locking or the dangers of inadvertently creating side effects
fn channels() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handlers = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });
    // wait call threads end
    for h in handlers {
        h.join().unwrap();
    }
    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_| rx.recv().unwrap()).collect();
    println!("channels: {:?}", numbers);
}

/// Other then sending data into threads one way, many programs operate on a shared state where
/// multiple execution streams have to access and change one or more shared variables. Typically,
/// this warrants a ```mutex```(short for ```mutual exclusion```互斥的), so that any time something
/// is accessed within this locked mutex, it's guaranteed to be a single thread.
///
/// This is an old concept and implemented in the Rust standard library. How does that facilitate
/// accessing a variable? Wrapping a variable into a Mutex type will provide for the locking mechanism,
/// thereby making it accessible from multiple concurrent writers. However, they don't have ownership
/// of that memory area yet.
///
/// In order to provide that ownership across threads, similar to what Rc does within a single thread,
/// Rust provides the concept of an ```Arc```, an ```atomic reference counter```. Using this Mutex on top,
/// it's the thread-safe equivalent of an Rc wrapping a RefCell, a reference counter that wraps a mutable container.
///
/// While the preferred way of doing concurrent programming is still to use immutable variables
/// as often as possible, safe Rust provides the tools for working with shared data without side effects.
fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handlers = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });
    for handle in handlers {
        handle.join().unwrap();
    }
    println!("shared state: {:?}", v.lock().unwrap());
}

fn sanitize(s: String) -> String {
    /// while this is akin to changing the value of a variable,
    /// shadowing does not replace mutability.
    ///
    /// Interior mutability:
    /// Can a variable be immutable and mutable at the same time? Of course.
    /// Boxed variables(Box, Rc and so on) are an immutable reference to the heap
    /// and they contain the actual value. For these kinds of containers, there is no reason why
    /// the inner variable cannot be changed - a task that can be done safely in Rust using RefCell.
    ///
    /// RefCell maintains single ownership of a value but allows mutable borrowing checked at runtime.
    /// Instead of compiler errors, violating the rules of borrowing will lead to a runtime panic!,
    /// crashing the problem.
    /// This entire concept is called ```interior mutability```(内部可变性) and is often used in combination
    /// with Rc in order to provide a value to multiple owners with mutability at will.
    /// Clearly, to provide a great user experience, it's strongly recommended to make sure
    /// the borrowing rules can't be violated in other ways.
    /// Wrapping a RefCell in an Rc acts as the gatekeeper for having multiple owners, including a way
    /// to change the contents. This is actually similar to more traditional programming languages
    /// such as Java or C#, where typically references are moved between method calls, pointing to
    /// the object;s instance on the heap memory.
    ///
    /// This pattern is very important for implementing complex programs and data structures, since
    /// ownership of a specific variable is not always clear.
    let s = s.trim();
    let s = s.replace(" ", "_");
    s
}

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    pub fn append(&mut self, value: String) {
        let new = Rc::new(RefCell::new(value));
        /*
        match self.tail.take() {
            Some(old) => {
                /// This borrow reference only lives as long as the assignment takes,
                /// thereby ruling out creating a too-large scope and violating the borrowing rules.
                /// By using the RefCell function's borrow_mut(), it will check for and enforce
                /// borrowing rules and panic in the case of a violation. Later on, we will also
                /// talk about the Mutex type, which is essentially a multithreaded version of these cells.
                old.borrow_mut().next = Some(new);
                old.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        }
        */
    }
}

#[cfg(test)]
mod tests {
    use crate::concurrency_and_mutability::{channels, shared_state, threading, threading_move};

    #[test]
    fn test_threading() {
        threading();
    }

    #[test]
    fn test_threading_move() {
        threading_move();
    }

    #[test]
    fn test_channels() {
        channels();
    }

    #[test]
    fn test_shared_state() {
        shared_state();
    }
}
