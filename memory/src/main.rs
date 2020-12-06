use std::time::Instant;

fn main() {
    let mut matrix = vec![vec![0; 100]; 100];
    let _res = 0;

    let mut sum = 0;
    for _k in 0..100 {
        let start_time = Instant::now();
        for i in 0..100 {
            for j in 0..100 {
                //  matrix[j][i] += 1;
                matrix[i][j] += 1;
            }
        }
        sum += start_time.elapsed().as_nanos();
    }
    println!("{}", sum / 100);
}
