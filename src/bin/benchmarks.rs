mod koelner_phonetik;
mod koelner_phonetik_threaded;
use std::time::{Duration, Instant};



fn threaded_vs_unthreaded_phonetics(){
    let string = "QWKEJHQWEKLJNQWKEJWNQEWQJKnkjdsnfjksdnfjskdfnsjkfnsdjknfsdkjnkj";
    let len = string.len();
    println!("Vergleich der Zeit, die verwendet wird, um den phonetischen Codes eines 'zufälligen' Wortes der Länge {} zu berechnen \n",len);

    let start = Instant::now();
    koelner_phonetik::get_phonetic_code(string);
    let duration = start.elapsed();
    println!("Time elapsed time in UNTHREADED phonetic calculation 1 is: {:?}", duration);

    let start = Instant::now();
    koelner_phonetik::get_phonetic_code(string);
    let duration = start.elapsed();
    println!("Time elapsed time in UNTHREADED phonetic calculation 2 is: {:?}", duration);

    let start = Instant::now();
    koelner_phonetik_threaded::get_phonetic_code(string);
    let duration = start.elapsed();
    println!("Time elapsed time in THREADED phonetic calculation 1 is: {:?}", duration);

    let start = Instant::now();
    koelner_phonetik_threaded::get_phonetic_code(string);
    let duration = start.elapsed();
    println!("Time elapsed time in THREADED phonetic calculation 2 is: {:?}", duration);
}

fn main() {
    threaded_vs_unthreaded_phonetics();
}