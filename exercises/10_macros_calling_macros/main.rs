////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;
use std::fmt::Debug;

fn print_pair<K: Debug, V: Debug>(pair: (K, V)) {
    println!("{pair:#?}");
}

fn print_hashmap<K: Debug, V: Debug>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}

macro_rules! pair {
    ($first: expr => $second: expr) => {
        ($first, $second)
    };
}

macro_rules! hashmap {
    ( $( $key: expr => $value: expr ,)* ) => {
        HashMap::from([
            $( pair!($key => $value), )*
        ])
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
