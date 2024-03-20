mod mutable;
mod scope;
mod variable;

use mutable::mutable;
use scope::scope;
use variable::variable;

fn main() {
    variable();
    mutable();
    scope();
    define_x();
    shadowed();
    remove_line();
}
// calling a function

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

/*
Shadowing...you can declare a new variable with the same name as a previous variable.
here we can say the first one has been shadowed by the second one
*/

fn shadowed() {
    let x: i32 = 5;

    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x: i32 = 42;
    println!("{}", x);
}

fn remove_line(){
   let mut x: i32 = 1;
   x = 7; // x = 7

   let mut x = x;
   x += 3;

   let y = 4;
   // shadowing
   let y = "I can also be bound to text!";
   println!("Success");
}
