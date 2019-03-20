fn main() {
    loop_result();
    loop_for();
    loop_with_range();
}

fn loop_result(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("counter: {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20, "Result não é 20");
}

fn loop_for(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn loop_with_range(){
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}