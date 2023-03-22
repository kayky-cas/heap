use heap::Heap;

fn main() {
    let mut heap = Heap::new();

    for i in [20, 2, 10, 7, 4, 9, 5, 3, 6, 89, 34235] {
        heap.insert(i);
    }

    println!("{}", heap);
}
