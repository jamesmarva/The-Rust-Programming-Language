fn main() {
    let mut a = [12,232,234,2,34,234,234,2,34,2,34,234,234,2,34,234,2,34,2,34,234,2,34,2];
    merge_sort_1(&mut a);
    for i in &a {
        println!("{}", i);
    }
}

pub fn merge_sort_1(arr: &mut [i32]) {
    // let x = [1, 2, 3];
    let mut tmp: Vec<i32> = Vec::new();
    let mut l = arr.len();
    while l > 0 {
        tmp.push(0i32);
        l-=1;
    }
    let len = arr.len();
    merge_core(arr, &mut tmp, 0, (len - 1) as usize);
}
fn merge_core(arr: &mut [i32], tmp: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    let mid = start + ((end - start) >> 1);
    merge_core(arr, tmp, start, mid);
    merge_core(arr, tmp, mid + 1, end);
    merge(arr, tmp, start, mid, end);
}

fn merge(arr: &mut [i32], tmp: &mut [i32], start: usize, mid: usize, end: usize) {
    let mut i1 = start;
    let mut i2 = mid + 1;
    let mut i = 0;
    while i1 <= mid && i2 <= end {
        if arr[i1] <= arr[i2] {
            tmp[i] = arr[i1];
            i1+=1;
        } else {
            tmp[i] = arr[i2];
            i2+=1;
        }
        i+=1;
    }
    while i1 <= mid {
        tmp[i] = arr[i1];
        i+=1;
        i1+=1;
    }
    while i2 <= end {
        tmp[i] = arr[i2];
        i+=1;
        i2+=1;
    }

    let len = i;
    i = 0;
    while i < len {
        arr[start + i] = tmp[i];
        i+=1;
    }
}
