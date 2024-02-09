# Builder pattern
The Builder Pattern is a creational design pattern that aims to separate the construction of a complex object from its representation, allowing the same construction process to create various representations. It provides a way to construct a complex object step by step, allowing the construction process to be controlled independently of the final object's structure.

## Intent:
Separate the construction of a complex object from its representation.
Construct an object step by step, allowing the same construction process to create different representations.

## Components:
Director: Responsible for orchestrating the construction process using the builder.
Builder: Abstract interface defining the steps for constructing an object.
Concrete Builders: Concrete implementations of the builder interface, each providing its own way to construct the object.
Product: The complex object being constructed.