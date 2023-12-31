struct FilterCondition<T>(T);

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.0 == *item
    }
}

fn custom_filter<T, I>(collection: I, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    collection.filter(|i| condition.is_match(i)).collect()
}

fn main() {
    let collection = vec![16, 128, 32, 64, 128, 512];
    let condition = FilterCondition(128);
    let result = custom_filter(collection.into_iter(), &condition);
    println!("Filtered result: {:?}", result);
}

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
