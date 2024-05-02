/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn aVeryBigSum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let ar: Vec<i64> = vec![1000000001, 1000000002, 1000000003, 1000000004, 1000000005];

    let result = aVeryBigSum(&ar);

    println!("{}", result);
}
