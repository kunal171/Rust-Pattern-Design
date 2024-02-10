fn main() {
    // define any variable
    let mut x = 5;

    // Borrow `x` -- but clone it first
    let y = &mut (x.clone());

    // without the x.clone() two lines prior, this line would fail on compile as
    // x has been borrowed
    // thanks to x.clone(), x was never borrowed, and this line will run.
    println!("{}", x);

    // perform some action on the borrow to prevent rust from optimizing this
    //out of existence
    *y += 1;

}
