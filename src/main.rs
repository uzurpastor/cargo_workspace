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


//------------ BEGIN EX 3 ----------//

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
//------------ END EX 3 ----------//

//------------ BEGIN EX 4 ----------//

fn vernam_chiper(){
	// 0110000 0011110 0010100

	let x: Vec<char> = "and".chars().collect();
	
	let b : Vec<u8> = [0b110000, 0b011110, 0b010100].to_vec();
	let mut r :Vec<u8> = Vec::new();
	for i in 0..x.len(){
		r.push(((x[i] as u8) ^ b[i]).try_into().unwrap());
	}
	for i in 0..r.len(){
		println!("{:?}: {:#010b} ^ {:#010b} = {:#010b} -> {:?}",
			x[i], 
			x[i] as u8, 
			b[i], 
			r[i],
			char::from_u32(r[i].into()).unwrap());
	}
}
//------------ END EX 4 ----------//

//------------ BEGIN EX 5 ----------//

// fn find_nicesize(_alfabet : &str) -> (u8, u8) {
// 	// let len :f32 = alfabet.len() as f32;
// 	let size: i8 = 35;
// 	let s : i8 = (size as f32).sqrt().round() as i8;
// 	let mut min_diff : i8 = size - s*s;

// 	let (mut x , mut y) = (s , s);

// 	'x:  loop {
// 		'y:  loop{
// 			min_diff = size - x*y;
// 			if min_diff == 0 {
// 				break 'x;
// 			} else {
// 				y += 1;
// 				if min_diff < size - x*y {
// 					min_diff = size - x*y;
// 					break 'y;
// 				}else
// 				if min_diff > size - x*y{
// 					break 'y;
// 				}
// 			}
// 		}
// 		x += 1;
// 		y = s;
		
// 	}


// 	(x as u8 , y as u8)
// }

fn create_square(alfabet_s : &str,  word_s : &str, size :(usize, usize)) -> Vec<Vec<char>> {
	//---BEGIN---//
		// create chiper alfabet which need recreate in square // 
		// main logic from fn 'create_key_alfabet' > ex3 //

	let alfabet : Vec<char> = alfabet_s.chars().collect();
    let word : Vec<char> = word_s.chars().collect();
    let mut key_alfabet : Vec<char> = Vec::new();

    {
	    let mut word_c : Vec<char> = word.clone();
	    key_alfabet.append(&mut word_c);
	}

    for i in 0..alfabet.len(){
    	let mut k : u8 = 0;
    	for j in 0..word.len(){
    		if alfabet[i] == word[j]{
    			continue;
    		}else {
    			k += 1;
    		}
    	}

    	if usize::from(k) == word.len(){
    		key_alfabet.push(alfabet[i]) 
    	};
    }
    //---END---//
	//---BEGIN---//
		// put element in form of square //

	let (x, y) = size;
    let mut square : Vec<Vec<char>> = vec!(vec!('#';y);x);

	let mut k = 0;
    'rows: for i in 0..x{
    	for j in 0..y{
	   		if key_alfabet.get(k) == None{
	    		break 'rows;
	    	}
	    	else{
	    		square[i][j] = key_alfabet[k];
	    	}
	    	k += 1;
    	}
    }

    {
    	for i in 0..x{
		    println!("{:?}", square[i]);
    	}
    	println!("-----------------");
    }
    square
    //---END---//
}
fn chip_word(key_word : &str, square_key : Vec<Vec<char>>, square_chiper : Vec<Vec<char>>) -> Vec<char>{
	let mut chip : Vec<char> = Vec::new();

	chip
}


//------------ END EX 5 ----------//

fn main(){
 //    cesar();
 //    aphinian_cesar_chiper();

 //    //------- BEGIN EX 3 ---------//
	// println!("cesar with keyword:");
	// {
	//     let step_for_chiper: usize = usize_input("input step value: ");
	//     let alfabet : &str = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ";
	//     let key_w : &str = "ЛІЦЕЙ";

	//     let key_a : Vec<char> = create_key_alfabet(&alfabet, step_for_chiper);
	//     let chiped_word : Vec<char> = chiper_cesarword(key_a, &key_w, alfabet);

	//     println!("{:?}\n{:?}", key_w, chiped_word);
	// }
	// //------- END EX 3 ------------//
 //    //------- BEGIN EX 4 ---------//
 //    println!("vernam chiper:");
 //    {
 //    	vernam_chiper();
 //    }
	//------- END EX 4 ------------//
    //------- BEGIN EX 5 ---------//
    println!("twosquare_chiper");
    {
    	let key_word : &str = "ЛІЦЕЙ";
    	let chiper_word : &str = "ФОРЕЛЬ";
		let alfabet : &str = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ.,";
		let size :(usize, usize) = (5 , 7); //find_nicesize(&alfabet);
		let square_key : Vec<Vec<char>> = create_square(&alfabet, &key_word, size);
		let square_chiper: Vec<Vec<char>> = create_square(&alfabet, chiper_word, size);
		let chipered_word: Vec<char> = chip_word(key_word, square_key, square_chiper);
		println!("{:?}", chipered_word);
    }
	//------- END EX 5 ------------//

}