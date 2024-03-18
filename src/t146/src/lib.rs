fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let res_a = vec![1; 1];
    let res_b = vec![1; 2];

    if num_rows == 1 {
        return vec![res_a];
    }

    if num_rows == 2 {
        return vec![res_a, res_b];
    }

    let mut res: Vec<Vec<i32>> = Vec::new();

    res.push(res_a);
    res.push(res_b);

    for i in 3..num_rows + 1 {
        let i = i as usize;
        let mut temp = vec![0; i];

        for j in 0..i {
            if j == 0 || j == i - 1 {
                temp[j] = 1;
            } else {
                let source = &res[i - 2];
                temp[j] = source[j - 1] + source[j];
            }
        }
        res.push(temp);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(generate(1), vec![vec![1]]);
        assert_eq!(generate(2), vec![vec![1], vec![1; 2]]);
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ]
        )
    }
}
