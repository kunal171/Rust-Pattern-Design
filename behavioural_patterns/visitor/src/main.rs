// Element interface
trait Visitable {
    fn accept(&self, visitor: &mut Visitor);
}

// Concrete elements
struct ElementA;
struct ElementB;

impl Visitable for ElementA {
    fn accept(&self, visitor: &mut Visitor) {
        visitor.visit_element_a(self);
    }
}

impl Visitable for ElementB {
    fn accept(&self, visitor: &mut Visitor) {
        visitor.visit_element_b(self);
    }
}

// Visitor interface
trait Visitor {
    fn visit_element_a(&mut self, element: &ElementA);
    fn visit_element_b(&mut self, element: &ElementB);
}

// Concrete visitor
struct ConcreteVisitor;

impl Visitor for ConcreteVisitor {
    fn visit_element_a(&mut self, element: &ElementA) {
        println!("ConcreteVisitor visited ElementA");
    }

    fn visit_element_b(&mut self, element: &ElementB) {
        println!("ConcreteVisitor visited ElementB");
    }
}

// Object structure
struct ObjectStructure {
    elements: Vec<Box<dyn Visitable>>,
}

impl ObjectStructure {
    fn new() -> Self {
        ObjectStructure {
            elements: Vec::new(),
        }
    }

    fn add_element(&mut self, element: Box<dyn Visitable>) {
        self.elements.push(element);
    }

    fn accept(&self, visitor: &mut Visitor) {
        for element in &self.elements {
            element.accept(visitor);
        }
    }
}

fn main() {
    let mut object_structure = ObjectStructure::new();
    object_structure.add_element(Box::new(ElementA));
    object_structure.add_element(Box::new(ElementB));

    let mut visitor = ConcreteVisitor;
    object_structure.accept(&mut visitor);
}
