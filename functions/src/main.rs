fn main() {
    println!("Hellow, world!");

    let y = {
        let x = 3;
        x + 1
    };

    let x = another_function(y);

    //another_function(y);

    println!("The value of the return type is: {}", x);
}

fn another_function(x: i32) -> i32 {
    println!("Another function.");

    if x == 5 {
        return x;
    }
    println!("The value of x is: {}", x);

    x
}
