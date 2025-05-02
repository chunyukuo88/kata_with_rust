mod first_word;
use first_word::get_first_word;

fn main() {
    let my_string = String::from("time for tea!");
    let result = get_first_word(&my_string);
    println!("{}", result);
}
