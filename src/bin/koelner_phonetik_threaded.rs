
use std::sync::{Arc, Mutex};
use std::thread;

pub fn get_phonetic_code(word:&'static str) -> String {
    let lowercase_word = word.to_lowercase();
    //Arc implementiert Sync und erlaubt mehrere Referenzen auf einen Wert
    //Mutex sorgt dafür dass nur ein Thread gleichzeitig einen Wert verändern kann. "mutually exclude"
    let code = Arc::new(Mutex::new(vec![]));

    //magic happens here
    for (i, c) in lowercase_word.chars().enumerate() {
        let code = code.clone();

        thread::spawn(move || {
            let beginning_of_word = i == 0;
            let mut code = code.lock().unwrap();

            if c == 'a' || c == 'e' || c == 'i' || c == 'j' || c == 'o' || c == 'u' || c == 'y' || c == 'ä' || c == 'ö' || c == 'ü' {
                code.push('0');
            } else if c == 'b' {
                code.push('1');
            } else if c == 'f' || c == 'v' || c == 'w' {
                code.push('3');
            } else if c == 'g' || c == 'k' || c == 'q' {
                code.push('4');
            } else if c == 'l' {
                code.push('5');
            } else if c == 'm' || c == 'n' {
                code.push('6');
            } else if c == 'r' {
                code.push('7');
            } else if c == 's' || c == 'z' || c == 'ß' {
                code.push('8');
            } else if c == 'd' || c == 't' {
                if is_next_letter(word, i, 'c') || is_next_letter(word, i, 'ß') ||
                    is_next_letter(word, i, 'z') ||
                    is_next_letter(word, i, 's') {
                    code.push('8');
                } else {
                    code.push('2');
                }
            } else if c == 'p' {
                if is_next_letter(word, i, 'h') {
                    code.push('3');
                } else {
                    code.push('1');
                }
            } else if c == 'x' {
                if is_previous_letter(word, i, 'c') ||
                    is_previous_letter(word, i, 'k') ||
                    is_previous_letter(word, i, 'q') {
                    code.push('8');
                } else {
                    code.push('4');
                    code.push('8');
                }
            } else if c == 'c' {
                if beginning_of_word {
                    if is_next_letter(word, i, 'a') ||
                        is_next_letter(word, i, 'h') ||
                        is_next_letter(word, i, 'k') ||
                        is_next_letter(word, i, 'l') ||
                        is_next_letter(word, i, 'o') ||
                        is_next_letter(word, i, 'q') ||
                        is_next_letter(word, i, 'r') ||
                        is_next_letter(word, i, 'u') ||
                        is_next_letter(word, i, 'x') {
                        code.push('4');
                    } else {
                        code.push('8');
                    }
                } else {
                    if is_previous_letter(word, i, 's') ||
                        is_previous_letter(word, i, 'z') ||
                        is_previous_letter(word, i, 'ß') {
                        code.push('8');
                    } else if is_next_letter(word, i, 'a') ||
                        is_next_letter(word, i, 'h') ||
                        is_next_letter(word, i, 'k') ||
                        is_next_letter(word, i, 'l') ||
                        is_next_letter(word, i, 'o') ||
                        is_next_letter(word, i, 'q') ||
                        is_next_letter(word, i, 'r') ||
                        is_next_letter(word, i, 'u') ||
                        is_next_letter(word, i, 'x') {
                        code.push('4');
                    } else {
                        code.push('8');
                    }
                }
            }
        });
    }

    //Unser phonetischer Code befindet sich in einem Vektor. Dieser wird jetzt in einen String umgewandelt.
    let mut phonetic_string = String::from("");
    let code = code.clone();
    let code = code.lock().unwrap();
    for i in 0..code.len() {
        phonetic_string.push(code[i]);
    }

    //Duplikate entfernen (phonetischer Algorithmus)
    let mut last_char = ' ';
    let mut phonetic_string_clone = String::new();
    for c in phonetic_string.chars() {
        if c != last_char {
            phonetic_string_clone.push(c);
        }
        last_char = c;
    }
    phonetic_string = phonetic_string_clone;

    //Entfernen der 0en (phonetischer Algorithmus)
    let slice = str::replace(&phonetic_string[1..], "0", "");
    phonetic_string = [&phonetic_string[0..1], &slice].join("");

    (phonetic_string)
}



fn is_next_letter (string:&str, i:usize, letter:char) -> bool {
    if string.len() > i+1 {
        let byte: u8 = string.as_bytes()[i + 1];
        return   letter == byte as char
    }
    return false
}

fn is_previous_letter (string:&str, i:usize, letter:char) -> bool {
    if i >= 1 {
        let byte: u8 = string.as_bytes()[i - 1];
        return    letter == byte as char
    }
    return false;
}


fn main(){
    println!("{}",(get_phonetic_code("testwort")));
}