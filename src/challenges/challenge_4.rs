use std::iter::FromIterator;
use utils::TrieNode;

pub fn execute(input: &str) {
    let result_a = challenge_a(input);
    println!("Challenge 4a: {}", result_a);
    let result_b = challenge_b(input);
    println!("Challenge 4b: {}", result_b);
}

fn challenge_a(input: &str) -> i32 {
    let passphrases = parse_array_input(input);
    count_valid_passphrases(passphrases)
}

fn challenge_b(input: &str) -> i32 {
    let passphrases = parse_array_input(input).iter().map(|passphrase| to_sorted_vec(passphrase)).collect();
    count_valid_passphrases(passphrases)
}

fn count_valid_passphrases(passphrases: Vec<Vec<String>>) -> i32 {
    let mut valid_count = 0;

    for passphrase in passphrases {
        let mut root_node = TrieNode::new();
        let mut has_dupes = false;

        for word in passphrase {
            has_dupes |= root_node.insert_check_dupe(word.as_bytes());
        }

        if !has_dupes {
            valid_count += 1;
        }
    }

    valid_count
}

fn parse_tabbed_data(input: &str) -> Vec<String> {
    let mut output = Vec::new();
    for word in input.split_whitespace() {
        output.push(word.to_string());
    }

    output
}

fn parse_array_input(input: &str) -> Vec<Vec<String>> {
    input.lines().map(parse_tabbed_data).collect()
}

fn to_sorted_vec(input: &[String]) -> Vec<String> {
    input.iter().map(|word| {
        let mut chars = word.chars().collect::<Vec<char>>();
        chars.sort();
        String::from_iter(chars)
    }).collect()
}