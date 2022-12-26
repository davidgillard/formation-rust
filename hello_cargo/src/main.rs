fn main() {
    greet_world();
}

fn greet_world() {
    let russian_language = "Всем привет";
    let french_language = "Bonjour tout le monde";
    let english_language = "Hello, world";
    let japan_language = "ハロー・ワールド";
    let chinese_language = "大家好";
    let regions = [
        russian_language,
        french_language,
        english_language,
        japan_language,
        chinese_language,
    ];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
