
fn array_subset(array_a: &[i32], array_b: &[i32]) -> bool {
    let m: i32 = array_a.len().try_into().unwrap();
    let n: i32 = array_b.len().try_into().unwrap();
    if m < n {
        return false;
    }           
    for arr in array_a.windows(array_b.len()) {
        if arr == array_b {
            return true;
        }
    }
    return false;
}

fn main() {
    let array_a: [i32; 8] = [1, 2, 3, 5, 6, 8, 10, 11];
    let array_b: [i32; 3] = [6,8,10];

    let result = array_subset(&array_a, &array_b);
    println!("Result  =====>>> {}", &result);
}

