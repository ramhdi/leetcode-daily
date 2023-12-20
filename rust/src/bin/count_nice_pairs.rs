// https://leetcode.com/problems/count-nice-pairs-in-an-array
// 2023/11/21

use std::collections::HashMap;

fn rev(num: i32) -> i32 {
    let mut result: i32 = 0;
    let mut num2: i32 = num;

    while num2 > 0 {
        result = result * 10 + num2 % 10;
        num2 /= 10;
    }
    return result;
}

fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut result: i32 = 0;

    let mut nums_reduced: Vec<i32> = vec![];
    for n in nums {
        nums_reduced.push(n - rev(n));
    }

    let mut count: HashMap<i32, i32> = HashMap::new();
    for &nr in &nums_reduced {
        result = (result + count.get(&nr).cloned().unwrap_or(0)) % (1_000_000_007);
        *count.entry(nr).or_insert(0) += 1;
    }
    return result;
}

fn main() {
    println!(
        "{}",
        count_nice_pairs(vec![
            8047408, 192867140, 497837845, 279787822, 151999002, 168514912, 193424242, 399636844,
            132424231, 476736524, 267958611, 493350382, 476382727, 232939232, 197000791, 295291645,
            126313621, 374645524, 7956597, 1376731, 496463745, 234481430, 359130803, 287625836,
            250572050, 42311324, 477434624, 493231448, 493231244, 150494051, 184645534, 365252413,
            495764582, 335976531, 384564332, 377151623, 198736741, 335161533, 245552540, 194897341,
            83911938, 220562020, 496645745, 287151782, 374635526, 372483324, 485101584, 271797172,
            244949442, 254333303, 251635002, 459181805, 472392123, 241350140, 256121502, 336895621,
            354635302, 358909704, 194525491, 3606063, 194150341, 63477436, 341936141, 60299206,
            69811896, 369928813, 229926920, 435310522, 299542980, 463777364, 164534512, 305885501,
            437181734, 74288247, 487281835, 171161022, 423966312, 496989544, 452633252, 252433101,
            141565141, 315895501, 478897927, 232532230, 472451262, 160504114, 476666674, 6179716,
            251483002, 474777474, 443532332, 475808424, 457514604, 400936002, 384878483, 172616122,
            283292232, 165645615, 392000144, 378636873
        ])
    );
}
