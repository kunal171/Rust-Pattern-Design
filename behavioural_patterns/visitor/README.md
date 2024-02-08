# Strategy 
The Strategy Design Pattern is a behavioral design pattern that allows you to define a family of algorithms, encapsulate each one of them, and make them interchangeable. This pattern lets the algorithm vary independently from clients that use it.

Here's how the Strategy Design Pattern works:

1 Define the Strategy Interface: Create an interface or an abstract class that declares a method to be used by the different algorithms.

2 Implement Concrete Strategies: Implement concrete classes that adhere to the strategy interface. Each concrete strategy represents a different algorithm.

3 Context Class: Create a context class that holds a reference to a strategy object. This class delegates the algorithm's execution to the strategy object.

4 Client Code: In client code, instantiate the context class with the desired strategy object. The client interacts with the context class, which internally invokes the algorithm provided by the selected strategy object.