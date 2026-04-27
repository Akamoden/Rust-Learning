use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let password_length = rng.random_range(14..=20); // generate a random length between 14 and 20 inclusive
    let random_password = generate_random_password(password_length);

    println!("Generated random password: {random_password}");
    println!("Sorted password: {}", sort_password(&random_password));
}
//todo : add a way to force capitle letters
fn generate_random_password(length: usize) -> String {
    let charset =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
         abcdefghijklmnopqrstuvwxyz";
    let spcial_charset = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    let charset = format!("{}{}{}", charset, spcial_charset, "0123456789");

    let chars: Vec<char> = charset.chars().collect();

    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let index = rng.random_range(0..chars.len());
            chars[index]
        })
        .collect()
}

fn sort_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();

    chars.sort_unstable_by_key(|&c| {
        let alpha_rank = if c.is_ascii_alphabetic() {
            c.to_ascii_lowercase() as u8
        } else {
            u8::MAX
        };

        let type_rank = if c.is_ascii_uppercase() {
            0u8
        } else if c.is_ascii_lowercase() {
            1u8
        } else if c.is_ascii_digit() {
            2u8
        } else {
            3u8
        };

        (alpha_rank, type_rank, c as u32)
    });

    chars.into_iter().collect()
}
