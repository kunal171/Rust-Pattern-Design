//! In Rust, when dealing with trait objects (e.g., using Box<dyn Trait>), 
//! the default behavior is to use heap allocation because trait objects have a dynamically unknown size at compile time. 
//! This allows for flexibility, but it introduces the overhead of heap allocation.
//! 
//! To achieve on-stack dynamic dispatch, Rust employs a technique known as "fat pointers" or "thin pointers," 
//! which involves storing additional information alongside the pointer to enable dynamic dispatch without resorting to heap allocation.


use std::io;
use std::fs;


/*Thin Pointers (On-Stack Dynamic Dispatch):
When a trait object consists only of a single trait, and that trait has no associated objects (associated types or lifetime bounds), 
Rust can use a "thin pointer" to store the trait object on the stack directly. A thin pointer consists of a data pointer and a vtable pointer. */

/*Fat Pointers (Default Dynamic Dispatch):
If the trait has associated types or lifetime bounds, Rust uses a "fat pointer," which includes both the data and the vtable. 
Fat pointers are larger than thin pointers and are typically allocated on the heap.
*/

trait MyTrait {
    fn do_something(&self);
}

struct MyStruct;

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something!");
    }
}

fn main() {
    let my_struct = MyStruct;
    let my_struct1 = MyStruct;

    // On-stack dynamic dispatch using a thin pointer
    let trait_object: &dyn MyTrait = &my_struct;

     // Default dynamic dispatch using a fat pointer
    let trait_object1: Box<dyn MyTrait> = Box::new(my_struct1);


    // Call the method through the trait object
    trait_object.do_something();

    trait_object1.do_something();
}
