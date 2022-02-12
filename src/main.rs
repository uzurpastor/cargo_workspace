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


fn cesar_init(word: &Vec<char>, alfabet: &Vec<char>, step: usize) -> Vec<char>{
   
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


fn cesar(){
    let alfabet: Vec<char> = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ"
        .chars()
        .collect();

    let word: Vec<char> = "КРИПТОГРАФІЯ"
        .chars()
        .collect();

    let step: usize = usize_input("input step for crypto"); 

    println!(" - {:?}", word);
    let cesar_word: Vec<char> = cesar_init(&word, &alfabet, step);
    println!(" + {:?}", cesar_word);
}

////////////////////////////////////////////

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

////////////////////////////////////////////

// fn logic_keyword_cesar_chiper(word: &Vec<char>, alfabet: &Vec<char>, k_value: usize) -> Vec<char>{
// }


fn keyword_cesar_chiper(){
    println!("cesar with keyword:");
    let k_value: usize = usize_input("input k value: ");

    let alfabet = vec!["АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ".chars()];
    let word = vec!["ЛІЦЕЙ".chars()];

    // let chiper_word: Vec<char> = logic_keyword_cesar_chiper(&word, &alfabet, k_value);
    let key_word = vec!["ФОРЕЛЬ".chars()];
    
    let key_alfabet = vec!["".chars()];
    for i in 0..alfabet.len()+k_value{
        if i < k_value{
            key_alfabet.add();

        }else if i >= k_value && i < k_value+key_word.len(){
            key_alfabet[i] = key_word[i - k_value];

        }else if i >= k_value+key_word.len(){
            let mut index_alfb: usize = 0;
            let mut ret_v = true;
            
            for j in key_word.iter(){
                if assert_eq!(alfabet[index_alfb], key_word[j]){ret_v = false}
            }

            if ret_v{
                key_alfabet[i%alfabet.len()] = alfabet[index_alfb];
            }

            index_alfb += 1;
        }

    }

}

/////////////////////////////////////////////

fn main(){
    cesar();
    aphinian_cesar_chiper();

}