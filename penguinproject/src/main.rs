use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;

fn main() {
    for input in std::env::args().skip(1) {
        match input.as_str() {
        "p-random" => print_random_facts(),
        "p-ordered" => print_ordered_facts(),
        "fchoice" => fast_food_choice(),

        "fcount" => {
         println!(
            "There are {} fast food restaurants.",
            count_fast_food_restaurants()
        );
    }

    "fdescribe" => {
        let mut rng = rand::rng();
        let choice = FAST_FOOD_RESTAURANTS.choose(&mut rng).unwrap();
        describe_fast_food(choice);
    }

    _ => eprintln!("Unknown argument: {input}"),
        }
    }
}

fn fast_food_choice() {
    let mut rng = rand::rng();
    let choice = FAST_FOOD_RESTAURANTS.choose(&mut rng).unwrap();
    println!("You should eat at: {choice}");
}
fn print_ordered_facts() {
    let facts = penguin_facts1();
    for fact in facts {
        println!("{fact}");
    }
}
fn print_random_facts() {
    let mut facts = penguin_facts1();

    let mut rng = rand::rng();

    facts.shuffle(&mut rng);

    for fact in facts {
        println!("{fact}");
    }
}
fn penguin_facts1() -> Vec<&'static str> {
    let penguin_facts1 = vec![
        "Penguins are flightless birds.",
        "They are excellent swimmers.",
        "Penguins live in the Southern Hemisphere.",
        "There are 18 species of penguins.",
        "The Emperor Penguin is the tallest and heaviest of all penguin species.",
        "Penguins have a layer of blubber to keep them warm in cold environments.",
        "They primarily eat fish, squid, and krill.",
        "Penguins are social animals and often live in large colonies.",
        "They have a unique way of communicating through vocalizations and body language.",
        "Penguins can drink saltwater because they have a gland that filters out the salt.",
        "Penguins are known for their distinctive black and white plumage, which provides camouflage while swimming.",
        "Penguins mate for life and often return to the same nesting sites each year.",

    ]; 
    penguin_facts1



}

static FAST_FOOD_RESTAURANTS: &[&str] = &[
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
];

fn count_fast_food_restaurants() -> usize {
    FAST_FOOD_RESTAURANTS.len()
}
fn describe_fast_food(name: &str) {
    let count = name.chars().count();
    println!("There are {count} letters: {name}");
}