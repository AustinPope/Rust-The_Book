const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of max points is: {}", MAX_POINTS);

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    let spaces = "hello";
    println!("The value of spaces is: {}", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let _spaces: bool = false;
    let _spaces = "hello";

    //array example

    //let a: [i32; 5];
    //println!("array at position 1 that is allocated but not init: {}", a[0]);
    //----BREAKS----- because a is not init, just a test

    let a = [1, 2, 3, 4];
    println!("array at position 1 that is allocated but not init: {}", a[0]);

    //let a = [1, 2, 3, 'a'];
    //-----BREAKS------ not a tuple, needs all same value

    let a = [3; 5];

    
    let tup = (500, 5.4, 1, 'a');
    let (_x, y, _z, _h) = tup;
    println!("This is destructing, y is equal to: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _var = x.0;

}
