use rand::Rng;
use std::fs::read_to_string;
use digest::Digest;
use sha2::Sha256;

fn main() {

    let lower_range:u128 = 1;
    let higher_range:u128 = 999999999999999999999999999999999999;

    let random_number = rand::thread_rng().gen_range(lower_range..higher_range);
    // println!("Random number: {}", random_number);

    let random_number_binary_padded = format!("{:0width$b}", random_number, width = 128);
    println!("{}", random_number_binary_padded);
    
    let mut hasher = Sha256::new();
    hasher.update(random_number_binary_padded.as_bytes());
    let result = hasher.finalize();

    let hash_hex = format!("{:x}", result);
    println!("SHA-256 hash: {}", hash_hex);
    
    // let checksum = &hash_hex[0..1];
    // println!("checksum is {:0width}", checksum);

    let checksum_binary = format!("{:0width$b}", u16::from_str_radix(&hash_hex[0..1], 16).unwrap(), width = 4);
    println!("binary checksum is {}", checksum_binary);

    println!("random_number_binary_padded + checksum is {}{}, random_number_binary_padded lenght is {}, and checksum_binary lenght is {}, total length is {}", random_number_binary_padded, checksum_binary, random_number_binary_padded.len(), checksum_binary.to_string().len(), random_number_binary_padded.len()+checksum_binary.to_string().len());

    let seed_with_checksum = format!("{}{}", random_number_binary_padded, checksum_binary);
    println!("seed with checksum is: \n{}", seed_with_checksum);

    let wordlist = read_lines("wordlist.txt");

    let mut seed_phrase_words_in_binary = Vec::new();
    let mut seed_phrase_words = Vec::new();

    for slice in 1..=12  {
        let slicer = (slice-1)*11;
        // println!("slice number is {}, slicer is at {}", slice, slicer);
        let slice_to_push = &seed_with_checksum[slicer..=slicer+10];
        seed_phrase_words_in_binary.push(slice_to_push);
        
        
        // seed_phrase_words.push(wordlist[usize::from_str_radix(slice_to_push,2).unwrap()])
        seed_phrase_words.push(&wordlist[usize::from_str_radix(slice_to_push,2).unwrap()])

    }

    println!("{:?}", seed_phrase_words_in_binary);
    println!("{}", seed_phrase_words_in_binary.len());

    // seed_phrase_words.push(&wordlist[233]);
    println!("{:?}", seed_phrase_words);
    // for slice 

    // println!("{:?}", wordlist[0])
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
