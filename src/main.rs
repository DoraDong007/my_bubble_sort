fn bubble_sort(arr: &mut [i32]) {
    let mut swapped;
    let mut n = arr.len();
    while n != 0 {
        swapped = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = i;
            }
        }
        n = swapped;
    }
}

fn main() {
    let mut numbers = [8, 10, 2, 4, 1];
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);
}