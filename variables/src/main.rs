fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is now: {}", x);

    const TEST_CONSTANT: u32 = 800_000;

    let tup = ("Hello world!", 123_456);
    let(text, number) = tup;
    let number = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

    let sum = my_function(11, 22);
    if sum < 10
    {
        println!("The sum is less than 10");
    }
    else if sum > 10
    {
        println!("The sum is greater than 10");
    }
    else
    {
        println!("The sum is equal to 10");
    }
    
    let mut counter = 0;
    let result = loop
    {
        counter += 1;
        if counter == 10
        {
            break counter;
        }
    };
    println!("The loop result is: {}", result);

    let a = [10, 20, 30, 40];
    for element in a.iter()
    {
        println!("{}", element);
    }

    for number in 1..4+1
    {
        println!("{}", number);
    }
}

fn my_function(x: i32, y: i32) -> i32
{
    println!("The sum of x is: {}", x);
    println!("The sum of y is: {}", y);
    x + y    
}