use std::string::String;

fn main() {
	let sentence = String::from("Hello world eyy");
	let first_word = get_first_word(&sentence);
	dbg!(first_word);
}

// TODO: make this function a oneliner
fn get_first_word(s: &String) -> String {
	let word_list: Vec<&str> = s.split(' ').collect();
	String::from(word_list[0])
}
