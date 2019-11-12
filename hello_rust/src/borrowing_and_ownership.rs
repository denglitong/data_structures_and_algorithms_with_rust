use std::rc::Rc;

/// The rules of ownership are as follows:
/// ```
/// 1. The owner of a value is variable
/// 2. At any time, only a single owner is allowed
/// 3. The value is lost once the owner goes out of scope
/// ```
/// Every variable is owned by exactly one scope at any time
/// Therefore the developers is forced to pass ownership as required
///
/// stack vs heap memory
///
/// Borrowing instead of copying the entire value, a reference to the original value is moved into
/// the new scope. Just like in real life, the value continues to be owned by the original scope;
/// scopes with a reference are just allowed to use it as it was provided. Of course, this comes with
/// drawbacks for mutability, and some functions will require ownership for technical and semantic
/// reasons, but it also have advantages such as a smaller memory footprint.
/// The rules of borrowing are as follows:
/// ```
/// 1. Owners can have immutable or mutable references, but not both
/// 2. There can be multiple immutable references, but only one mutable reference
/// 3. Reference cannot be invalid
/// ```
/// Borrowed variables heavily on lifetimes. The most basic lifetimes is the scope it was created in.
/// However, if a reference should go into a struct field, how can the compiler know that the underlying
/// value has not been invalidated? The answer is explicit lifetimes!
///
/// If an input parameter is passed into the function and returned at the end, its lifetime surpasses
/// the functions's. While the function owns this part of the memory during its lifetime, it cannot
/// borrow a variable for longer than it actually exists. There are several solutions to this problem:
/// 1. Change the type definition to require ownership, that is, we ```move ownership``` for which returned
/// 2. Clone to ```pass ownership```
///
/// Lifetimes cause a lot of strange errors for many Rust users, with the 2018 edition, the introduction
/// of non-lexical lifetimes, make it less to worry about, the borrow checker able to check semantically
/// whether the variable was used. Compile does not just check the beginning and ending of a scope,
/// but also if the reference was used at all.
///
/// A single ownership is powerful, but it does not work for every use case. Large objects or shared objects
/// that other instances need to own are examples, and immutable ownership makes life easier for the condition.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //no_ref_counter();
        ref_counter();
    }
}

/// A reference counter(std::rc::Rc<T>) encapsulates a variable of type T allocated on the heap
/// and returns an immutable reference when created. The reference can be cloned with low overhead(
/// it's only a reference count that is incremented) but never transformed into a mutable reference.
/// Regardless, it acts just like owned data, passing through function calls and property lookups.
///
/// This approach works great for single-thread and immutable scenarios, but will refuse to compile
/// multithreaded code.
#[derive(Debug)]
struct FileNameRc {
    name: Rc<String>,
    ext: Rc<String>,
}

fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    for _ in 0..3 {
        println!(
            "{:?}",
            FileNameRc {
                name: name.clone(),
                ext: ext.clone()
            }
        )
    }
}

#[derive(Debug)]
struct FileName {
    name: String,
    ext: String,
}

fn no_ref_counter() {
    let name = String::from("main");
    let ext = String::from("rs");

    for _ in 0..3 {
        // println!("{:?}", FileName { name, ext })
    }
}
