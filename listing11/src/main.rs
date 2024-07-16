use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Person {
    given_name: String,
    last_name: String,
}

pub fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
}

fn main() {
    let bob = Person {
        given_name: "Bob".to_owned(),
        last_name: "Roberts".to_owned(),
    };

    dbg!(full_name(&bob.given_name, &bob.last_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let person: Person =
            serde_json::from_str("{\"given_name\": \"Bob\", \"last_name\": \"Roberts\" }").unwrap();

        assert_eq!(
            person,
            Person {
                given_name: "Bob".to_owned(),
                last_name: "Roberts".to_owned()
            }
        );
    }
}
