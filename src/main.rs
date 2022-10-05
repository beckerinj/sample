#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
struct Value {
    pub timestamp: i64,
    pub value: String,
}

fn main() {
    // take the Values in these two vecs
    // concatenate together balances with same timestamp using intermediary hashmap
    // sort ending vector after collectiong from hashmap
    // print single resulting vec of Value

    let nums_1 = vec![
        Value { timestamp: 0, value: "15".to_string() },
        Value { timestamp: 3, value: "10".to_string() },
        Value { timestamp: 4, value: "8".to_string() },
        Value { timestamp: 6, value: "19".to_string() },
    ];

    let nums_2 = vec![
        Value { timestamp: 1, value: "5".to_string() },
        Value { timestamp: 3, value: "51".to_string() },
        Value { timestamp: 4, value: "14".to_string() },
        Value { timestamp: 7, value: "6".to_string() },
    ];

    let all_nums = vec![nums_1, nums_2];

    let mut map: HashMap<i64, String> = HashMap::new();

    for nums in all_nums.iter() {
        for bal in nums {
            map.insert(bal.timestamp, bal.value);
        }
    }

    let result_nums: Vec<Value> = map
        .iter()
        .map(|(timestamp, value)| Value{ timestamp, value })
        .collect();

    println!("{result_nums:#?}");
}
