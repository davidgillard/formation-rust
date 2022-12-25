fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = five(); // appel de la fonstion 5
    println!("La valeur de x est: {}", x);

    let x = plus_one(5);
    println!("La valeur de x est: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("La valeur de x is: {}", x);
    println!("La valeur de y is: {}", y);
}

// creation de la fonction five
fn five() -> i32 {
    5
}

// creation de la fonction plus_one
fn plus_one(x: i32) -> i32 {
    x + 1
}
