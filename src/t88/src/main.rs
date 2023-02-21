struct Solution {}

impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        let mut res = 1;

        let mut coins = coins;
        coins.sort();

        for val in coins {
            if val > res {
                break;
            }
            res += val;
        }

        res
    }
}

fn main() {
    let s = Solution {};
    let res = s.get_maximum_consecutive(vec![1, 2, 1, 3]);
    println!("the value is {res}");
}
