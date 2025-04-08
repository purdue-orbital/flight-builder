fn heapify<V: PartialOrd>(arr: &mut [V], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn build_heap<V: PartialOrd>(arr: &mut [V]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
}

pub fn heap_sort<V: PartialOrd>(arr: &mut [V]) {
    let n = arr.len();
    build_heap(arr);

    for i in (0..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        heap_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }
}
