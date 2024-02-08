# RAII with guards


RAII (Resource Acquisition Is Initialization) with guards is a programming pattern used to manage resources in a scoped and deterministic way. It's commonly used in Rust, where ownership and lifetimes play a significant role in resource management.

## Here's how RAII with guards works:

1 Resource Acquisition: When a resource (such as a file handle, mutex lock, or memory allocation) needs to be acquired, it is obtained within a scoped block or function.

2 Initialization: The resource is acquired and initialized within a scope, usually at the beginning of a block or function.

3 Guard Creation: A guard object is created to manage the acquired resource. The guard is responsible for releasing the resource when it goes out of scope, ensuring proper cleanup.

4 Scoped Resource Usage: The acquired resource is used within the scope where it's acquired, ensuring that it's released automatically when it's no longer needed, even in the presence of early returns or panics.

5 Automatic Cleanup: When the guard object goes out of scope (either through normal control flow or unwinding due to a panic), its destructor is called, releasing the resource it manages.

# Motivation
Where a resource must be finalised after use, RAII can be used to do this finalisation. If it is an error to access that resource after finalisation, then this pattern can be used to prevent such errors.

# Advantages
Prevents errors where a resource is not finalised and where a resource is used after finalisation.

# Discussion
RAII is a useful pattern for ensuring resources are properly deallocated or finalised. We can make use of the borrow checker in Rust to statically prevent errors stemming from using resources after finalisation takes place.

The core aim of the borrow checker is to ensure that references to data do not outlive that data. The RAII guard pattern works because the guard object contains a reference to the underlying resource and only exposes such references. Rust ensures that the guard cannot outlive the underlying resource and that references to the resource mediated by the guard cannot outlive the guard. To see how this works it is helpful to examine the signature of deref without lifetime elision:
```Rust
fn deref<'a>(&'a self) -> &'a T {
    //..
}
```
The returned reference to the resource has the same lifetime as self ('a). The borrow checker therefore ensures that the lifetime of the reference to T is shorter than the lifetime of self.

Note that implementing Deref is not a core part of this pattern, it only makes using the guard object more ergonomic. Implementing a get method on the guard works just as well.