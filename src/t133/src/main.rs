impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut points = [0; 1000];

        for trip in trips.iter() {
            let (passengers, from, to) = (trip[0], trip[1], trip[2]);
            for i in from..to {
                points[i as usize] += passengers;
                if points[i as usize] > capacity {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {}
