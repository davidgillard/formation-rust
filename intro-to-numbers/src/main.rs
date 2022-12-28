fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21; // ici on ajoute une annotation i32
    let twenty_two = 22i32; // l'anotation de type suffixe

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000; // les underscores ne sont pas interpretés par le
                                      // compilateur et augmente la lisibilité
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{:02}", forty_twos[0]);
}
