// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>()+chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
   // let it =words.iter();
   let mut results :Vec<String> =vec![];

    for i in words.iter(){
        let mut c =i.chars();
        match c.next(){
            None => results.push(" ".to_string()),
            Some(first)=>{
                let rest =c.as_str();
                results.push(format!("{}{}",first.to_uppercase(),rest));
            }
        }
    }
    results
    // ???
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let mut results :String=Default::default();
    for i in words.iter(){
        let mut c = i.chars();
        match c.next(){
            None =>results.push(' '),
            Some(first)=>{
                let _rest=c.as_str();
                results.push_str(&first.to_uppercase().to_string());
                results.push_str(c.as_str());
            }
        }

    }
    results
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
