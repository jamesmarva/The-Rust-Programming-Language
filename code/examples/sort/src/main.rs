use sort::merge_sort_mod;

fn main() {
    let mut a = [12,232,234,2,34,234,234,2,34,2,34,234,234,2,34,234,2,34,2,34,234,2,34,2];
    // quick(&mut a);
    merge_sort_mod::merge_sort_1(&mut a);
    for item in &a {
        println!("{}", item);
    }
}

fn quick(arr: &mut [i32]) {
    quick_sort(arr, 0, (arr.len() - 1) as usize);
}

fn quick_sort(arr: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let pivot_idx = partition(arr, start, end);
    quick_sort(arr, start, pivot_idx - 1);
    quick_sort(arr, pivot_idx + 1, end);
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let pivot_val = arr[start];
    let mut left = start;
    let mut right = end + 1;
    loop {
        loop {
            left += 1;
            if arr[left] > pivot_val || left == end {
                break;
            }
        }
        loop {
            right -= 1;
            if arr[right] <= pivot_val || right == start {
                break;
            }
        }
        if left >= right {
            break;
        }
        swap(arr, left, right);
    }
    swap(arr, start, right);
    right
}

fn swap(arr: &mut [i32], a: usize, b: usize) {
    let tmp = arr[a];
    arr[a] = arr[b];
    arr[b] = tmp;
}
