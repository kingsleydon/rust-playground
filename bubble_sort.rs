use std::cmp::PartialOrd;

fn bubble_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        for j in 1..n - i {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
            }
        }
    }
}

fn main() {
    let mut a = [333, 4444, 1, 22, 55555];
    bubble_sort(&mut a);
    println!("{:?}", a);
}
