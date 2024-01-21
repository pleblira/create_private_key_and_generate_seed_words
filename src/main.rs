use digest::generic_array::ArrayLength;
use digest::generic_array::GenericArray;
use rand::Rng;
use std::u128;
use std::{fs::read_to_string, process::exit};
// use digest::Digest;
use sha2::Sha256;
use sha2::Digest;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    // let lower_range:u128 = 1;
    // let higher_range:u128 = 999999999999999999999999999999999999;

    let random_number = rand::thread_rng().gen_range(u128::MIN..u128::MAX);
    println!("Random number in hex: {}", random_number);
    println!("Random number in binary: {:b}", random_number);
    
    let random_number_binary_padded = format!("{:0width$b}", random_number, width = 128);
    println!("random number in binary padded to 128 chars is {}", &random_number_binary_padded);

    // uncomment the line below to use a preset hex seed instead of randomizing
    // let random_number_binary_padded: String = format!("{:0width$b}",u128::from_str_radix("0c1e24e5917779d297e14d45f14e1a1a",16).unwrap(), width=128);

    // uncomment the line below to use a preset 128-bit binary number instead of randomizing
    // let random_number_binary_padded: String = format!("{:0}","01000100010100110111001000010010110010001101011010000000101000011011101010000111111101000111010101011110001010010011011010010111");

    let mut hasher = Sha256::new();
    hasher.update(&random_number_binary_padded);
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);
    println!("SHA-256 hash of random number: {:x}", result);
        
    let checksum_binary = format!("{:0width$b}", u16::from_str_radix(&hash_hex[0..1], 16).unwrap(), width = 4);
    println!("binary checksum is {}", checksum_binary);

    let seed_with_checksum = format!("{}{}", random_number_binary_padded, checksum_binary);
    println!("seed with checksum is {}\n", seed_with_checksum);

    // println!("random_number_binary_padded + checksum is {}{}, random_number_binary_padded length is {}, and checksum_binary length is {}, total length is {}", random_number_binary_padded, checksum_binary, random_number_binary_padded.len(), checksum_binary.to_string().len(), random_number_binary_padded.len()+checksum_binary.to_string().len());

    // let seed_with_checksum = format!("{}{}", random_number_binary_padded, checksum_binary);
    // println!("seed with checksum is: \n{}", seed_with_checksum);

    let wordlist = read_lines("wordlist.txt");

    let mut seed_phrase_words_in_binary = Vec::new();
    let mut seed_phrase_words = Vec::new();

    for slice in 1..=12  {
        let slicer = (slice-1)*11;
        let slice_to_push = &seed_with_checksum[slicer..=slicer+10];
        // println!("slice number is {}, slicer is at {}, slice to push is {}", slice, slicer, &slice_to_push);
        println!("{}", &seed_with_checksum);
        let spaces = std::iter::repeat(" ").take(slicer).collect::<String>();
        println!("{}{}", spaces, slice_to_push);
        seed_phrase_words_in_binary.push(slice_to_push);
        seed_phrase_words.push(&wordlist[usize::from_str_radix(slice_to_push,2).unwrap()-1])
    }

    println!("{:?}", seed_phrase_words_in_binary);
    println!("{:?}", seed_phrase_words);

}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
