use std::collections::HashMap;

pub fn play() {

    // strings
    let s =  "नमस्ते";
    println!("{}", s);
    ::strings::print_chars(s);
    println!("---");
    ::strings::print_graphmems(s);


    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
