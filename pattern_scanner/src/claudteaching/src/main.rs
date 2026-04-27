//todo : make a static a-z list
fn main() { 
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let shift = -2;
    let encrypted = ceasar_cipher (input.trim(), shift);
    println!("Encrypted: {}", encrypted);
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];


fn ceasar_cipher (input:&str, shift: i32) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if ALPHABET.contains(&c) {
            let index = ALPHABET.iter().position(|&x| x == c).unwrap() as i32;
            let new_index = (index + shift).rem_euclid(ALPHABET.len() as i32);
            result.push(ALPHABET[new_index as usize]);
        } else {
            result.push(c);
        }
    }
    result
}

