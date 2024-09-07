/*
Coutils by Alyx Shang.
Licensed under the FSL v1.
*/

/// Returns a vector of strings from strings 
/// split by some character. Both the string
/// and split character have to be strings.
pub fn clean_split(
    subject: &str, 
    split_char: &str
) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in subject.split(&*split_char) {
        let new_item: String = item.to_string();
        result.push(new_item);
    }
    return result;
}
