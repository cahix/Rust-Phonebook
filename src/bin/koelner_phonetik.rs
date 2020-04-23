

   pub fn get_phonetic_code(word: &str) -> String {
        let lowercase_word = word.to_lowercase();
        let mut code = String::from("");

        for (i, c) in lowercase_word.chars().enumerate() {
                let beginning_of_word = i == 0;

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
        }

        //remove duplicates
        let mut last_char = ' ';
        let mut code_clone = String::new();
        for c in code.chars() {
            if c != last_char {
                code_clone.push(c);
            }
            last_char = c;
        }
        code = code_clone;

        let slice = str::replace(&code[1..], "0", "");
        code = [&code[0..1], &slice].join("");

        (code)
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


   fn replace_char_at(s: &str, idx: usize, c: char) -> String{
       let mut r = String::with_capacity(s.len());
       for (i, d) in s.char_indices() {
           r.push(if i == idx { c } else { d });
       }
       r
   }





fn main(){
    println!("{}",get_phonetic_code("testwort"));
}