fn main() {
    println!("Hello, world!");
    let array: Vec<i32> = vec![3,2,8,9,1,4,6,0,5,7];
    quicksort(array);

}

fn quicksort(array:Vec<i32>) {
    let pivot = array[array.len()-1];
    let mut medium: Vec<i32> = array.clone();
    for n in 0..=medium.len() {
        if medium[n] < pivot {
            medium.insert(0, medium[n]);
            medium.remove(n);
        } else {
            medium.insert(medium.len()-1, medium[n]);
            medium.remove(n);
        }
    }
}