
fn array_subset(array_a: &[i32], array_b: &[i32]) -> bool {
    let m: i32 = array_a.len().try_into().unwrap();
    let n: i32 = array_b.len().try_into().unwrap();

    if m < n {
        return false;
    }       
    
    for i in 0..n - 1 {
        for j in 0..m-1 {
            if array_b[i as usize] == array_a[j as usize] {
                break;
            }
        }
        if i == m {
            return false;
        }
    }
    return true;
}

fn main() {
    let array_a: [i32; 8] = [1, 2, 3, 5, 6, 8, 10, 11];
    let array_b: [i32; 3] = [6,8,11];

    let result = array_subset(&array_a, &array_b);
    println!("ket qua  =====>>> {}", &result);
}

