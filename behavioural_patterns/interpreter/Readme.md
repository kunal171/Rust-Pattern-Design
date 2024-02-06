# Interpreter
## Description
If a problem occurs very often and requires long and repetitive steps to solve it, then the problem instances might be expressed in a simple language and an interpreter object could solve it by interpreting the sentences written in this simple language.

Basically, for any kind of problems we define:

A domain specific language,
A grammar for this language,
An interpreter that solves the problem instances.
# Motivation
Our goal is to translate simple mathematical expressions into postfix expressions (or Reverse Polish notation) For simplicity, our expressions consist of ten digits 0, …, 9 and two operations +, -. For example, the expression 2 + 4 is translated into 2 4 +.

Context Free Grammar for our problem
Our task is translating infix expressions into postfix ones. Let’s define a context free grammar for a set of infix expressions over 0, …, 9, +, and -, where:

Terminal symbols: 0, ..., 9, +, -
Non-terminal symbols: exp, term
Start symbol is exp
And the following are production rules
```
exp -> exp + term
exp -> exp - term
exp -> term
term -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```
NOTE: This grammar should be further transformed depending on what we are going to do with it. For example, we might need to remove left recursion. For more details please see Compilers: Principles,Techniques, and Tools (aka Dragon Book).

The Interpreter pattern is a behavioral design pattern that defines a way to evaluate language grammar or expressions. It's typically used when you have a language to interpret, and you want to provide a way to evaluate sentences in that language.

Intent:
Define a language grammar and provide a way to interpret sentences in that language.
Represent grammar rules as objects and provide an interface to interpret sentences.
Enable the interpreter to parse and interpret expressions in the language.
Participants:
Abstract Expression: Defines an interface for interpreting the language.
Terminal Expression: Implements the interface for terminal symbols or elements in the grammar.
Non-terminal Expression: Implements the interface for non-terminal symbols or elements in the grammar.
Context: Contains information that needs to be interpreted.
Client: Builds the abstract syntax tree representing the language and invokes the interpreter to interpret expressions.
