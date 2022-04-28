
// fn find_nicesize(_alfabet : &str) -> (u8, u8) {
//  // let len :f32 = alfabet.len() as f32;
//  let size: i8 = 35;
//  let s : i8 = (size as f32).sqrt().round() as i8;
//  let mut min_diff : i8 = size - s*s;

//  let (mut x , mut y) = (s , s);

//  'x:  loop {
//      'y:  loop{
//          min_diff = size - x*y;
//          if min_diff == 0 {
//              break 'x;
//          } else {
//              y += 1;
//              if min_diff < size - x*y {
//                  min_diff = size - x*y;
//                  break 'y;
//              }else
//              if min_diff > size - x*y{
//                  break 'y;
//              }
//          }
//      }
//      x += 1;
//      y = s;
        
//  }


//  (x as u8 , y as u8)
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
fn chip_word(word : &Vec<char>, square_ch1 : Vec<Vec<char>>, square_ch2 : Vec<Vec<char>>) -> Vec<char>{
    let mut chip : Vec<char> = Vec::new();
    let mut i = 0;
    loop{
        if i+1 >= word.len(){
            break;
        }
        let (x1, y1) = find_pos(word[i], &square_ch1);
        i += 1;
        let (x2, y2) = find_pos(word[i], &square_ch2);
        chip.push(square_ch1[x1][y2]);
        chip.push(square_ch2[x2][y1]);
        i += 1;
    }
    chip
}


fn find_pos(symbol : char, square: &Vec<Vec<char>>) -> (usize, usize){
    let (mut x, mut y) = (-1, -1);
    'a:for i in 0..square.len(){
         for j in 0..square[i].len(){
            if symbol == square[i][j]{
                x = i as isize;
                y = j as isize;
                break 'a;
            }

        }
    }
    if (x, y) == (-1, -1){
        panic!("dont find symbol in chiper square");
    }
    (x.try_into().unwrap(), y.try_into().unwrap())
    
}


fn main() {
    println!("twosquare_chiper");
    
    let word: Vec<char> = "ХЛОПЕЦЬ.".chars().collect();
    if word.len() % 2 != 0{
        panic!("bad word");
    }
    let chiper_word1 : &str = "ФОРЕЛЬ";
    let chiper_word2 : &str = "ЛІЦЕЙ";
    let alfabet : &str = "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ.,";
    let size :(usize, usize) = (7 , 5); //find_nicesize(&alfabet);
    let square_ch1 : Vec<Vec<char>> = create_square(&alfabet, chiper_word1, size);
    let square_ch2 : Vec<Vec<char>> = create_square(alfabet, chiper_word2, size);
    let chipered_word: Vec<char> = chip_word(&word, square_ch1, square_ch2);
    println!("{:?}", word);
    println!("{:?}", chipered_word);

}
