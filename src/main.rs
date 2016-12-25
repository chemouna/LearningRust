
fn main() {
    variable_binding_example();
}

fn mutation_example () {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let mut guess = String::new();
}

fn variable_binding_example () {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
}

