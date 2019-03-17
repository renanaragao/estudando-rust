fn main() {
    another_function(5, 6);

    statement_vs_expressions();

    println!("The value of x is: {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_vs_expressions() {

    //statement
    {
        let x = 3;
        let x = x + 1;
        println!("The value of statement is: {}", x);
    }

    let expression = {
        let x = 3;
        x + 1
    };

    println!("The value of expression is: {}", expression);
}

fn five() -> i32 {
    5
}