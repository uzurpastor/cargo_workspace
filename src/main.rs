use std::io;


fn usize_input(prompt: String) -> usize{

    let mut input_text = String::new();
    loop{
        println!("{prompt}");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => return i,
            Err(..) => println!("this was not an integer, try again"),
        };
    }
}


fn cesar_init(word: Vec<char>, alfabet:Vec<char>, step: usize) -> Vec<char>{
   
    let strong_of_alfabet = alfabet.len();
    let mut ret: Vec<char> = ""
        .chars()
        .collect();

    for i_chars in word.iter() {
        for j in 0..alfabet.len(){
            if i_chars == &alfabet[j]{
                ret.push(alfabet[(j+step)%strong_of_alfabet]);
            }
        }
    }
    ret
}


fn main() {

    let alfabet: Vec<char> = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ"
        .chars()
        .collect();

    let word: Vec<char> = "КРИПТОГРАФІЯ"
        .chars()
        .collect();

    let step: usize = usize_input("input step for crypto".to_string()); 

    println!(" - {:?}", word);
    let cesar_word: Vec<char> = cesar_init(word, alfabet, step);//костыль
    println!(" + {:?}", cesar_word);
}