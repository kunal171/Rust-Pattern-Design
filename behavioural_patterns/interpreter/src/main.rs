pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,  // Iterator over characters of the input string
}

impl<'a> Interpreter<'a> {

    // Constructor method to create a new Interpreter instance
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }  // Initialize the iterator with characters of the input string
    }

    // Method to get the next character from the input string iterator
    fn next_char(&mut self) -> Option<char> {
        self.it.next()  // Returns the next character or None if end of input is reached
    }

    // Method to interpret the infix expression and convert it to postfix notation
    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);  // Process the first term of the expression

        // Continue processing the expression until all characters are consumed
        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {  // If the character is an operator
                self.term(out);  // Process the next term
                out.push(op);  // Append the operator to the output string
            } else {
                panic!("Unexpected symbol '{}'", op);  // Panic if an unexpected symbol is encountered
            }
        }
    }

    // Method to process a term in the expression
    fn term(&mut self, out: &mut String) {
        // Get the next character from the input iterator and process it
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),  // If the character is a digit, append it to the output string
            Some(ch) => panic!("Unexpected symbol '{}'", ch),  // Panic if an unexpected symbol is encountered
            None => panic!("Unexpected end of string"),  // Panic if the end of input is reached prematurely
        }
    }
}

// Main function to test the Interpreter implementation
pub fn main() {
    // Test case 1: Evaluate "2+3" infix expression
    let mut intr = Interpreter::new("2+3");
    let mut postfix = String::new();
    intr.interpret(&mut postfix);  // Convert infix expression to postfix
    assert_eq!(postfix, "23+");  // Check if the resulting postfix expression is correct

    // Test case 2: Evaluate "1-2+3-4" infix expression
    intr = Interpreter::new("1-2+3-4");
    postfix.clear();
    intr.interpret(&mut postfix);  // Convert infix expression to postfix
    assert_eq!(postfix, "12-3+4-");  // Check if the resulting postfix expression is correct
}
