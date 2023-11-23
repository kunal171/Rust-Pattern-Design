//! In Rust, the [non_exhaustive] attribute is used to indicate that a struct or enum is intentionally designed to be non-exhaustive. 
//! This means that new fields or variants may be added in the future without breaking backward compatibility. 
//! When combined with private fields, it can provide a way to extend a type without exposing the internal details to external code.

#[non_exhaustive]
struct MyStruct {
    field1: i32,
    field2: String,
    // Private field for extensibility
    _extensible: (),
}

impl MyStruct {
    pub fn new(field1: i32, field2: String) -> Self {
        MyStruct {
            field1,
            field2,
            _extensible: (),
        }
    }

    // Public method for accessing the extensible part
    pub fn extensible(&self) -> &() {
        &self._extensible
    }
}

mod a {
    // Public struct.
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }
    
    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String }
    }
}

fn print_matched_variants(s: a::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let a::S { foo: _, ..} = s;
    
    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("it's an A"),
        a::AdmitMoreVariants::VariantB => println!("it's a b"),

        // .. required because this variant is non-exhaustive as well
        a::AdmitMoreVariants::VariantC { a, .. } => println!("it's a c"),

        // The wildcard match is required because more variants may be
        // added in the future
        _ => println!("it's a new variant")
    }
}

fn main() {
    let my_instance = MyStruct::new(42, "Hello".to_string());

    // Accessing public fields
    println!("field1: {}", my_instance.field1);
    println!("field2: {}", my_instance.field2);

    // Accessing the extensible part
    let extensible_part = my_instance.extensible();
    println!("Extensible part: {:?}", extensible_part);
}
