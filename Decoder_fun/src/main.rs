
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: grep <word>");
        return;
    }
    let word = &args[0];
    let found_words = grep_to_vector(word, FIND_WORD);
    if !found_words.is_empty() {
        println!("Found words: {:?}", found_words);
    } else {
        println!("No words found containing: {word}");
    }

}


fn grep_to_vector<'a, T: AsRef<str>>(word: &str, items: &'a [T]) -> Vec<&'a str> 
where
    T: AsRef<str>,
{
    items.iter().filter(|item| item.as_ref().contains(word)).map(|item| item.as_ref()).collect()
}

static FIND_WORD: &[&str] = &[
    "McDonald's",
    "Burger King",
    "Wendy's",
    "Taco Bell",
    "KFC",
    "Subway",
    "Pizza Hut",
    "Dunkin' Donuts",
    "Chick-fil-A",
    "Sonic Drive-In",
    "Arby's",
    "Popeyes Louisiana Kitchen",
    "Chipotle Mexican Grill",
    "in-n-out Burger",
    "Five Guys",
    "Panda Express",
    "WORD"
];




