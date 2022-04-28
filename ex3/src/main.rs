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


fn set_key_word(default : &str) -> Vec<char>{
    println!("Input word for chipering [default = \"{}\"]: ", default);
    let mut input_text = String::new();
    io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

    let trimmed = input_text.trim();
        match trimmed.is_empty() {
            true => default.chars().collect(),
            false => trimmed.chars().collect(),
        }
}


fn create_key_alfabet(alfabet_s : &str, step: usize) -> Vec<char>{ 

    let key : Vec<char> = set_key_word("ФОРЕЛЬ");    
    let alfabet : Vec<char>= alfabet_s.chars().collect();

    let mut key_c : Vec<char> = key.clone();
    let mut key_alfabet : Vec<char> = Vec::new();

    for _i in 0..step{
        key_alfabet.push('#');
    }

    key_alfabet.append(&mut key_c);

    for i in 0..alfabet.len(){
        let mut k : u8 = 0;
        for j in 0..key.len(){
            if alfabet[i] == key[j]{
                continue;
            }else {
                k += 1;
            }
        }

        if usize::from(k) == key.len(){
            key_alfabet.push(alfabet[i]) 
        };
    }

    for i in 0..step{
        key_alfabet[i] = key_alfabet[key_alfabet.len() - step + i];
    }
    key_alfabet.truncate(key_alfabet.len() - step);

    key_alfabet

}

fn chiper_cesarword(key_a : Vec<char>, keystr_w : &str, alfabetstr: &str) -> Vec<char>{

    let key_w : Vec<char>= keystr_w.chars().collect();
    let alfabet : Vec<char>= alfabetstr.chars().collect();

    let mut chiped_word : Vec<char> = Vec::new();
    for i in 0..alfabet.len(){
        for j in 0..key_w.len(){
            if key_w[j] == alfabet[i]{
                chiped_word.push(key_a[i]);
            }
        }
    }
    chiped_word

}


fn main() {
    println!("cesar with keyword:");
    
    let step_for_chiper: usize = usize_input("input step value: ");
    let alfabet : &str = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ";
    let key_w : &str = "ЛІЦЕЙ";

    let key_a : Vec<char> = create_key_alfabet(&alfabet, step_for_chiper);
    let chiped_word : Vec<char> = chiper_cesarword(key_a, &key_w, alfabet);

    println!("{:?}\n{:?}", key_w, chiped_word);

}
