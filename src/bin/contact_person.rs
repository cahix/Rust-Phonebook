mod koelner_phonetik;
use serde::{Serialize, Deserialize};
use koelner_phonetik::get_phonetic_code;

#[derive(Debug, Deserialize, Eq, PartialEq,Serialize)]
pub struct Contact {
    pub name: String,
    pub phone_number_1: String,
    pub phone_number_2: String,
    pub phone_number_3: String,
    pub street: String,
    pub zip: String,
    pub town: String,
}
impl Contact {
}

#[derive(Debug)]
pub struct Person  {
    pub name: String,
}

trait has_phonetic {
    fn calc_phonetic(&self) -> String;
}

impl has_phonetic for Person {
    fn calc_phonetic(&self) -> String {
        (get_phonetic_code(&self.name))
    }
}

impl Person {
    pub fn calc_phonetic(&self) -> String {
        (get_phonetic_code(&self.name))
    }
}





//===========================================================================\\
fn main(){
    let p:Person = Person {name:"Jabba".to_string()};
    println!("{}",p.calc_phonetic());
}



