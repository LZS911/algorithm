impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        rides.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());
        let mut st = vec![];
        let calc = |ride: &[i32]| (ride[1] - ride[0] + ride[2]) as i64;
        let mut ret = 0;
        for ride in rides {
            let (start, end, mut profit) = (ride[0], ride[1], calc(ride.as_slice()));
            let pp = st.partition_point(|&(_, end)| end <= start);
            profit += match pp {
                0 => 0, // all > start
                _ => st[pp - 1].0,
            };
            if st.is_empty() || profit > st.last().unwrap().0 {
                if !st.is_empty() && end == st.last().unwrap().1 {
                    st.last_mut().unwrap().0 = profit; // replace
                } else {
                    st.push((profit, end));
                }
            }
            // eprintln!("{:?}", st);
            ret = ret.max(st.last().unwrap().0);
        }
        ret
    }
}
