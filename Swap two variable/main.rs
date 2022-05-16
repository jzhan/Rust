fn main() {
    let mut one = 2;
    let mut two = 5;

    println!("Before: {} {}", one, two);

    swap(&mut one, &mut two);

    println!("After : {} {}", one, two);

    std::mem::swap(&mut one, &mut two);

    println!("Origin: {} {}", one, two);
}

fn swap<T: Clone>(num_one: &mut T, num_two: &mut T) {
    let temp = (*num_one).clone();
    *num_one = (*num_two).clone();
    *num_two = temp;
}
