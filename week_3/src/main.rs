struct FilterCondition<T>(T);

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.0 == *item
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::FilterCondition;

    #[test]
    fn filter_condition_is_match_should_compare_properly() {
        let condition = FilterCondition("Polkadot");
        assert!(condition.is_match(&"Polkadot"));
        assert!(!condition.is_match(&"Ethereum"));
    }
}
