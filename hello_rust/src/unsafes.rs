/// ```unsafe``` is a keyword that can do most of the things the C programming language would let you do
/// + Dereference a raw pointer
/// + Call an unsafe function or method
/// + Access or modify a mutable static variable
/// + Implement an unsafe trait
///
/// unsafe:
/// + Dereferencing null, dangling, or unaligned pointers
/// + Reading uninitialized memory
/// + Breaking the pointer aliasing rules
/// + Producing invalid primitive values:
///     Dangling/null references
///     Null fn pointers
///     An undefined enum discriminant
///     A char outside the ranges [0x0, 0xD7FF] and [0xE000, 0X10FFFF]
///     A non-UTF8 string
/// + Unwinding into another language
/// + Causing a data race

#[cfg(test)]
mod tests {}
