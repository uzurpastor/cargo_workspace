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

fn main() {
    println!("vernam chiper:");

    vernam_chiper();
}
