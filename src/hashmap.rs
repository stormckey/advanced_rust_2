use std::collections::HashMap;

macro_rules! hashmap{
    ( $( $k:expr => $v:expr ),*) => {
        {
            let mut hash_map = HashMap::new();
            $(
                hash_map.insert($k,$v);
            )*
            hash_map
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashmap_macro() {
        let h = hashmap! {
            1 => "1",
            2 => "2",
            3 => "3"
        };
        assert_eq!(h[&1], "1");
        assert_eq!(h[&2], "2");
        assert_eq!(h[&3], "3");
    }
}
