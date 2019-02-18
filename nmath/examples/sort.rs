const ARRAY_A: [u32; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const ARRAY_B: [u32; 15] = [2, 1, 15, 4, 3, 5, 6, 13, 14, 7, 8, 10, 9, 11, 12];
const ARRAY_C: [u32; 15] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
const ARRAY_D: [u32; 15] = [1, 2, 1, 3, 2, 4, 3, 5, 4, 4, 5, 6, 6, 1, 0];

fn bubble_sort(array: &mut [u32]) {
    loop {
        let mut same = true;
        for j in 1..array.len() {
            if array[j - 1] > array[j] {
                array.swap(j - 1, j);
                same = false;
            }
        }
        if same {
            break;
        }
    }
}

fn selection_sort(array: &mut [u32]) {
    for j in 0..array.len() {
        let mut least = j;
        for i in j+1..array.len() {
            if array[i] < array[least] {
                least = i;
            }
        }
        array.swap(j, least);
    }
}

fn insertion_sort(array: &mut [u32]) {
    for j in 1..array.len() {
        let key = array[j];
        let mut i = j;
        while i != 0 && array[i - 1] > key {
            array[i] = array[i - 1];
            i = i - 1;
        }
        array[i] = key;
    }
}

fn merge_sort(array: &mut [u32]) {
    if array.len() > 1 {
        let q = array.len() >> 1;
        merge_sort(&mut array[..q]);
        merge_sort(&mut array[q..]);
        // Merge.
        let buffer = array.to_vec();
        let mut i = 0;
        let mut j = q;
        let mut k = 0;
        loop {
            // Find the lowest value.
            if j == array.len() || (buffer[i] <= buffer[j] && i < q) {
                array[k] = buffer[i];
                k += 1;
                i += 1;
            } else {
                array[k] = buffer[j];
                k += 1;
                j += 1;
            }
            // Exit loop.
            if k == array.len() {
                break;
            }
        }
    }
}

fn heap_sort(array: &mut [u32]) {
    #[allow(unused)]
    fn parent(i: usize) -> usize {
        i >> 1
    }
    fn left(i: usize) -> usize {
        i << 1
    }
    fn right(i: usize) -> usize {
        left(i) + 1
    }
    fn max_heapify(array: &mut [u32], i: usize) {
        let l = left(i);
        let r = right(i);
        // Left child exists and is > this node
        let mut largest = if l <= array.len() && array[l - 1] > array[i -1] {
            l
        } else {
            i
        };
        if r <= array.len() && array[r - 1] > array[largest - 1] {
            largest = r;
        }
        // Now has found index of largest value
        if largest != i {
            array.swap(i - 1, largest - 1);
            max_heapify(array, largest);
        }
    }
    fn build_max_heap(array: &mut [u32]) {
        let mut i = array.len() >> 1;
        loop {
            max_heapify(array, i);
            if i == 1 {
                break;
            }
            i -= 1;
        }
    }
    build_max_heap(array);
    let mut i = array.len() - 1;
    loop {
        array.swap(0, i);
        max_heapify(&mut array[..i], 1);
        i -= 1;
        if i == 0 {
            break;
        }
    }
}

fn quick_sort(array: &mut [u32]) {
    fn partition(array: &mut [u32]) -> usize {
        // Last index.
        let x = array[array.len() - 1];
        let mut i = 0;
        for j in 1..array.len() {
            if array[j - 1] <= x {
                i += 1;
                array.swap(i - 1, j - 1);
            }
        }
        array.swap(i, array.len() - 1);
        return i;
    }

    if array.len() > 1 {
        let q = partition(array);
        quick_sort(&mut array[..q]);
        quick_sort(&mut array[(q+1)..]);
    }
}

/// Sort on numbers from 0..=k
fn counting_sort(array: &mut [u32], k: u32) {
    let mut count = vec![0; k as usize + 1];
    // Count each element
    for j in 0..array.len() {
        count[array[j] as usize] += 1;
    }
    // Make it a cumulative count
    for j in 1..count.len() {
        count[j] += count[j - 1];
    }
    // Copy source array
    let source = array.to_vec();
    let mut j = array.len() - 1;
    loop {
        let b = count[source[j] as usize] as usize;
        array[b - 1] = source[j];
        count[source[j] as usize] -= 1;
        if j == 0 {
            break;
        }
        j -= 1;
    }
}

fn radix_sort(array: &mut [u32]) {
    let mut count;
    // Sort 4 bits at a time.
    for i in 0..8 {
        count = [0; 16];
        let bit_shift = i << 2;
        // Count each element on the 4 bit radix.
        for j in 0..array.len() {
            count[(array[j] as usize >> bit_shift) % 16] += 1;
        }
        // Make it a cumulative count
        for j in 1..count.len() {
            count[j] += count[j - 1];
        }
        // Copy source array
        let source = array.to_vec();
        let mut j = array.len() - 1;
        loop {
            let b = count[(source[j] as usize >> bit_shift) % 16] as usize;
            array[b - 1] = source[j];
            count[(source[j] as usize >> bit_shift) % 16] -= 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

fn check(array_a: [u32;15], array_b: [u32;15], array_c: [u32;15], array_d: [u32;15]) {
    assert_eq!(array_a, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(array_b, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(array_c, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    assert_eq!(array_d, [0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6, 6]);
}

fn main() {
    // Check insertion sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    insertion_sort(&mut array_a);
    insertion_sort(&mut array_b);
    insertion_sort(&mut array_c);
    insertion_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check merge sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    merge_sort(&mut array_a);
    merge_sort(&mut array_b);
    merge_sort(&mut array_c);
    merge_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check heap sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    heap_sort(&mut array_a);
    heap_sort(&mut array_b);
    heap_sort(&mut array_c);
    heap_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check quick sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    quick_sort(&mut array_a);
    quick_sort(&mut array_b);
    quick_sort(&mut array_c);
    quick_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check counting sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    counting_sort(&mut array_a, 15);
    counting_sort(&mut array_b, 15);
    counting_sort(&mut array_c, 15);
    counting_sort(&mut array_d, 15);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check radix sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    radix_sort(&mut array_a);
    radix_sort(&mut array_b);
    radix_sort(&mut array_c);
    radix_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check selection sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    selection_sort(&mut array_a);
    selection_sort(&mut array_b);
    selection_sort(&mut array_c);
    selection_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);

    // Check bubble sort.
    let mut array_a = ARRAY_A;
    let mut array_b = ARRAY_B;
    let mut array_c = ARRAY_C;
    let mut array_d = ARRAY_D;
    bubble_sort(&mut array_a);
    bubble_sort(&mut array_b);
    bubble_sort(&mut array_c);
    bubble_sort(&mut array_d);
    println!("{:?}\n{:?}\n{:?}\n{:?}", array_a, array_b, array_c, array_d);
    check(array_a, array_b, array_c, array_d);
}
