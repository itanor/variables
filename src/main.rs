fn main() {
    // mutable var
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // immutable var
    let y = 10;
    println!("The value of y is: {}", y);

    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const value: {}", THREE_HOURS_IN_SECONDS);

    // shadowing vars
    let will_be_shadowed = 10;

    let will_be_shadowed = will_be_shadowed + 1;
    {
        let will_be_shadowed = will_be_shadowed * 2;
        println!("The value of will_be_shadowed in the inner scope is: {}", will_be_shadowed);
    }
    println!("The value of x is: {}", will_be_shadowed);

    // change type of var when using 'mut' modifier
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // will not compile - already assigned a type...
    //let mut spaces = "    ";
    //spaces = spaces.len();

}
