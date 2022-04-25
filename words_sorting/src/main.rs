use std::fs;
use std::cmp;
use std::path::Path;

fn main() {
    let path = Path::new("./texts/words.txt");
    let mut content = String::new();

    if let Ok(text) = fs::read_to_string(&path) {
        let mut words: Vec<&str> = text.split(" ").collect();
        println!("words: {:?}", words);

        words.sort_by(|a, b| compare(a.to_lowercase(), b.to_lowercase()));
        words.iter().for_each(|x| {
            content.push_str(x);
            content.push_str("\n");
        });

        println!("sorted: {:?}", words);
    }

    let result_path = Path::new("./texts/words_sorted.txt");
    fs::write(&result_path, content.as_bytes()).unwrap();

    println!("sorted words are saved in `{}`", result_path.to_str().unwrap());
}

pub fn compare(a: String, b: String) -> cmp::Ordering {
    let b_bytes = b.as_bytes().to_vec();
    
    for (index, value) in a.as_bytes().into_iter().enumerate() {
        {
            if index >= b_bytes.len() {
                return cmp::Ordering::Greater;
            } else if value == &b_bytes[index] {
                continue;
            } else if value > &b_bytes[index] {
                return cmp::Ordering::Greater;
            } else {
                return cmp::Ordering::Less;
            }
        }
    }
    cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use std::cmp;

    #[test]
    fn test_compare() {
        assert_eq!(
            super::compare("them".to_owned(), "they".to_owned()),
            cmp::Ordering::Less
        );

        assert_eq!(
            super::compare("they".to_owned(), "them".to_owned()),
            cmp::Ordering::Greater
        );

        assert_eq!(
            super::compare("they".to_owned(), "they".to_owned()),
            cmp::Ordering::Equal
        );

        assert_eq!(
            super::compare("they".to_owned(), "".to_owned()),
            cmp::Ordering::Greater
        );
    }
}
