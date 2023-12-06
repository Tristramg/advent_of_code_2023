/*
distance = speed * (total_time - pressing_time)
speed = pressing_time
distance = pressing_time * total_time - pressing_time * pressing_time

solve
-pressing_time * pressing_time + pressing_time * total_time - distance >= 0
x = pressing_time
-x² + t·x - d = 0 (ax² + bx + c ; a=-1 b=t, c=-d)
*/

fn solve(time: f64, record: f64) -> u64 {
    let delta = time * time - 4.0 * record; // delta = t² - 4·d (b² - ac)
    assert!(delta >= 0.0);
    let x1 = (time - delta.sqrt()) / 2.0;
    let x2 = (time + delta.sqrt()) / 2.0;

    x2.floor() as u64 - x1.ceil() as u64 + 1
}

fn main() {
    let run1 = solve(51.0, 377.9) * solve(69.0, 1171.9) * solve(98.0, 1224.9) * solve(78.0, 1505.9);
    println!(
        "First star: {run1}, second star: {}",
        solve(51_699_878.0, 377_117_112_241_505.0)
    );
}

#[test]
fn test_solve() {
    assert_eq!(4, solve(7.0, 9.0));
    assert_eq!(8, solve(15.0, 40.0));
}
