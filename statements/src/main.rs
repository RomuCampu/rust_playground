fn main() {
    if_statements();

    loop_staements();

    while_statements();

    for_loop_statements();
}

fn for_loop_statements() {
    for number in (1..=5).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF");
}

fn while_statements() {
    let a = [1, 2, 3, 4, 5, 6, 7];

    for element in a.iter() {
        println!("The value of a is: {}", element);
    }

    let mut i = 0;

    while i < 7 {
        println!("The value of a is: {}", a[i]);

        i += 1;
    }

    let mut num = 3;

    while num != 0 {
        println!("The value of num is: {}!", num);

        num -= 1;
    }

    println!("LIFTOFFF!!!");
}

fn loop_staements() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the value of result is {}", result);
}

fn if_statements() {
    let number = 5;

    if number > 5 {
        println!("Condition was true");
    } else if number < 5 {
        println!("Conditon was false");
    } else {
        println!("Don't know what to do");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}", number);
}
