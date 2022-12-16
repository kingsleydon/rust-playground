fn sum(numbers: &[u32]) -> Option<u32> {
    let mut result = 0;

    for &number in numbers {
        if result > std::u32::MAX - number {
            return None;
        }
        result += number;
    }

    Some(result)
}

fn main() {
    let numbers1 = [1, 2, 3, 4];
    let numbers2 = [1, std::u32::MAX];

    println!("{:?}", sum(&numbers1));
    println!("{:?}", sum(&numbers2));
}
