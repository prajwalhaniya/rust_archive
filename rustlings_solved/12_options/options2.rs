fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target: &'static str = "rustlings";
        let optional_target = Some(target);

        // Using if-let statement
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // Using while-let statement with nested pattern matching
        while let Some(integer) = optional_integers.pop() {
            if let Some(i) = integer {
                assert_eq!(i, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}