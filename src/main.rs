fn main() {
    let text = "Rawr";
    let text_array = text.chars();
    let colors: Vec<u8> = Vec::with_capacity(3);
    for i in text_array.into_iter() {
        println!("{}", i as u8);
    }
}
