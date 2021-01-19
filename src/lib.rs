#![allow(unused)]
fn election(candidates: Vec<String>, votes: Vec<String>) -> String {
    String::from("Winner")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_election() {
        let candidates = vec![String::from("A"), String::from("B"), String::from("C")];
        let votes = vec![
            String::from("Voter A"),
            String::from("Voter B"),
            String::from("Voter C"),
        ];
        let result = election(candidates, votes);
        assert_eq!(String::from("A"), result);
    }
}
