// constants are hard coded and type must always be annotated
// can be declared in any scope, even global
// can't use "mut" with constants
// const MAX_POINTS: u32 = 100_000;

// can't reassign immutable variable
// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// adding "mut" makes x mutable
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// Shadowing is different from declaring "mut"
// performs transformation while keeping variable immutable
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     let x = x * 2;

//     println!("The value of x is : {}", x);
// }

// Shadowing allows us to chnage the type of the value, but reuse the same name
// "mut" can't be used because we can't mutate a variable's type
fn main() {
    let spaces = "    ";

    let spaces = spaces.len();

    println!("We have {} spaces!", spaces);
}