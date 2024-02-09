# Struct decomposition for independent borrowing
## Description

Sometimes a large struct will cause issues with the borrow checker - although fields can be borrowed independently, sometimes the whole struct ends up being used at once, preventing other uses. A solution might be to decompose the struct into several smaller structs. Then compose these together into the original struct. Then each struct can be borrowed separately and have more flexible behaviour.

This will often lead to a better design in other ways: applying this design pattern often reveals smaller units of functionality.

