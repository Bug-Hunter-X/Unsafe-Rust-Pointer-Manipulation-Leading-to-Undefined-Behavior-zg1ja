# Unsafe Rust Pointer Manipulation Bug

This repository demonstrates a common error in Rust involving unsafe code and vector manipulation. The code attempts to modify the contents of a vector using a raw pointer, resulting in undefined behavior because the vector's memory could be freed before the pointer is dereferenced.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file provides a safe and correct solution.

## Bug Description
The primary issue is the use of `v.as_mut_ptr()` to obtain a raw pointer, followed by direct modification of the pointed-to data without ensuring the vector's lifetime.  Once the `v` vector goes out of scope, its memory is deallocated, causing a use-after-free scenario.  This leads to unpredictable behavior such as crashes or data corruption.