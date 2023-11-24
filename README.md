# Rust-Pattern-Design

In software development, we often come across problems that share similarities regardless of the environment they appear in. Although the implementation details are crucial to solve the task at hand, we may abstract from these particularities to find the common practices that are generically applicable.

Design patterns are a collection of reusable and tested solutions to recurring problems in engineering. They make our software more modular, maintainable, and extensible. Moreover, these patterns provide a common language for developers, making them an excellent tool for effective communication when problem-solving in teams.

Design patterns in Rust
Rust is not object-oriented, and the combination of all its characteristics, such as functional elements, a strong type system, and the borrow checker, makes it unique. Because of this, Rust design patterns vary with respect to other traditional object-oriented programming languages. That’s why we decided to write this book. We hope you enjoy reading it! The book is divided in three main chapters:

Idioms: guidelines to follow when coding. They are the social norms of the community. You should break them only if you have a good reason for it.
Design patterns: methods to solve common problems when coding.
Anti-patterns: methods to solve common problems when coding. However, while design patterns give us benefits, anti-patterns create more problems.

## Idioms
Idioms are commonly used styles, guidelines and patterns largely agreed upon by a community. Writing idiomatic code allows other developers to understand better what is happening.

After all, the computer only cares about the machine code that is generated by the compiler. Instead, the source code is mainly beneficial to the developer. So, since we have this abstraction layer, why not make it more readable?

Remember the KISS principle: “Keep It Simple, Stupid”. It claims that “most systems work best if they are kept simple rather than made complicated; therefore, simplicity should be a key goal in design, and unnecessary complexity should be avoided”.

## Design Patterns
Design patterns are “general reusable solutions to a commonly occurring problem within a given context in software design”. Design patterns are a great way to describe the culture of a programming language. Design patterns are very language-specific - what is a pattern in one language may be unnecessary in another due to a language feature, or impossible to express due to a missing feature.

If overused, design patterns can add unnecessary complexity to programs. However, they are a great way to share intermediate and advanced level knowledge about a programming language.