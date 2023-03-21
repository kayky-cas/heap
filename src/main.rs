use heap::Heap;

fn main() {
    let mut heap = Heap::new();

    for i in [20, 2, 10, 7, 4, 9, 5, 3, 6, 89, 34235] {
        heap.insert(i);
    }

    println!("{}", heap);

    let _ = heap.get();

    println!("{}", heap);

    let heapsort: Heap<_> = vec![
        974, 707, 699, 483, 588, 592, 234, 129, 119, 363, 328, 57, 247, 548, 17, 23, 101,
    ]
    .into();

    println!("{}", heapsort);
}
