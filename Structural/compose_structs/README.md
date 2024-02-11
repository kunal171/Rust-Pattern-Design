# Struct decomposition for independent borrowing
## Description

Sometimes a large struct will cause issues with the borrow checker - although fields can be borrowed independently, sometimes the whole struct ends up being used at once, preventing other uses. A solution might be to decompose the struct into several smaller structs. Then compose these together into the original struct. Then each struct can be borrowed separately and have more flexible behaviour.

This will often lead to a better design in other ways: applying this design pattern often reveals smaller units of functionality.

# Motivation
This pattern is most useful, when you have a struct that ended up with a lot of fields that you want to borrow independently. Thus having a more flexible behaviour in the end.

# Advantages
Decomposition of structs lets you work around limitations in the borrow checker. And it often produces a better design.

# Disadvantages
It can lead to more verbose code. And sometimes, the smaller structs are not good abstractions, and so we end up with a worse design. That is probably a ‘code smell’, indicating that the program should be refactored in some way.

# Discussion
This pattern is not required in languages that don’t have a borrow checker, so in that sense is unique to Rust. However, making smaller units of functionality often leads to cleaner code: a widely acknowledged principle of software engineering, independent of the language.

This pattern relies on Rust’s borrow checker to be able to borrow fields independently of each other. In the example, the borrow checker knows that a.b and a.c are distinct and can be borrowed independently, it does not try to borrow all of a, which would make this pattern useless.