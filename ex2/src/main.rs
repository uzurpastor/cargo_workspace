use std::io;


fn usize_input(prompt: &str) -> usize{

    let mut input_text = String::new();
    loop{
        println!("{}", prompt.to_string());
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


fn aphinian_cesar_chiper_logic(word: &Vec<char>, alfabet: &Vec<char>, k_value: usize, a_value: usize)-> Vec<char> {
    let n_value = alfabet.len();
    let mut ret: Vec<char> = ""
        .chars()
        .collect();

    for i_chars in word.iter() {
        for j in 0..alfabet.len(){
            if i_chars == &alfabet[j]{
                let chiper_char: char = alfabet[((k_value*j)+a_value)%n_value];
                ret.push(chiper_char);
            }
        }
    }
    ret
}


fn aphinian_cesar_chiper(){
    println!("aphinian cesa:");
    let k_value:usize = usize_input("\tinput k value: ");
    let a_value:usize = usize_input("\tinput a value: ");

    let alfabet: Vec<char> = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ"
        .chars()
        .collect();

    let word: Vec<char> = "КРИПТОГРАФІЯ"
        .chars()
        .collect();
    
    let cesar_word: Vec<char> = aphinian_cesar_chiper_logic(&word, &alfabet, k_value, a_value);

    println!("{:?}", word);
    println!("{:?}", cesar_word);
}


fn main() {
    aphinian_cesar_chiper();
}
