
//ЗАЛИШИТЬ МІСТО
//ДИПЛОМ

    // let ch =  'А' as u32; // mount for ansii russia alf
    // for i in 0..33{
    //     println!("{:#10b} {}", ch+i, char::from_u32((ch+i).into()).unwrap());

    // }

fn create_key(message: &str, key_word: &str) -> Vec<char>{
    let key_word: Vec<char> = key_word.chars().collect();
    let mut key : Vec<char> = Vec::new();
    for i in 0..message.len(){
        key.push(key_word[i % key_word.len()]);
    }
    key
}

fn find_char(symbol: char, kit: &Vec<char>) -> usize{
    for i in 0..kit.len(){
        if kit[i] == symbol{
            return i;
        }
    }
    panic!("symbol {symbol} not find in the kit");
}

fn chipering(message: Vec<char>, key: Vec<char>, mode: char) -> Vec<char>{
    let alfabet : Vec<char> = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ".chars().collect();
    // let message : Vec<char> = message.retain(|&c| c == ' ');
    let mut chip : Vec<char> = Vec::new();
    for i in 0..message.len(){
        if message[i] == ' '{
            continue;
        }
        
        let n;
        if mode == 'p'{
            n = (find_char(message[i], &alfabet) + find_char(key[i], &alfabet)) % alfabet.len();
        }else{ // only 'n'
            let mut k;
            k = (find_char(message[i], &alfabet) as isize)
                         - (find_char(key[i], &alfabet) as isize);
            if k < 0{
            k = alfabet.len() as isize
                + (find_char(message[i], &alfabet) as isize)
                - (find_char(key[i], &alfabet) as isize);
            }
            n = k as usize % alfabet.len();
        }
        
        chip.push(
            alfabet[n]);
    }
    chip
}


fn main() {
    let message : &str = "ЗАЛИШИТЬМІСТО";
    let key_word: &str = "ДИПЛОМ";

    let key = create_key(&message, key_word);
    let message : Vec<char> = message.chars().collect();
    println!("{:?}", &message);
    let chip = chipering(message, key.clone(), 'p');
    println!("{:?}", &chip);
    let dechip = chipering(chip, key, 'n');
    println!("{:?}", dechip);

}
