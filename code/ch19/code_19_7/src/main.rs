use std::slice;

fn main() {
    // let address = 0x01234usize;

    // let r = address as *mut i32;
    // let mut arr: &[i32] = &[1];
    // arr = &mut arr;
    // // let a = &mut arr;

    // let ptr = arr.as_mut_ptr(); 

    // let s: &[i32] = unsafe {
    //     // slice::from_raw_parts_mut(r, 1)
    //     slice::from_raw_parts_mut(ptr, 100)
    // };

    // for ele in s {
    //     println!("ele is {}", ele);
    // }
    // test2();
    // test3();
    // test4() ;
    // test5()
    unsafe {
        test0()
    }
    
}

unsafe fn test0() {
    let add = 112;
    let ptr = add as *mut i32;
    
    let s: &mut [i32] = unsafe {
        slice::from_raw_parts_mut(ptr, 11)
    };

    for i in s.iter() {
        println!("i : {}", i);
    }
}


fn test1() {
    let address = 0x0123456789usize;
    let r = address as *mut i32;
     
    let s: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 1000)
    };
}

fn test2() {
    let mut arr = [1, 2, 3, 4, 5];
    let arr_reference_mutable = &mut arr;
    let raw_pointer = arr_reference_mutable.as_mut_ptr();

    let s: &[i32] = unsafe {
        slice::from_raw_parts_mut(raw_pointer, 1000)
    };

    for i in s {
        println!("i: {}", i);
    }
}


fn test3() {
    // 为什么这里要声明为mut？这个主要是因为 下面的 as_mut_ptr，得到的是一个mut pointer，
    // 为什么要mut pointer，因为下面的 from_raw_parts_mut 传的参数必须是一个mut
    // 不用mut行不行？再看一个例子,答案是可以的。
    let mut arr = [1, 2, 3];
    let arr_slice = &mut arr;
        // let first_element_of_array_reference_mutable = &mut arr[1];
    let row_pointer_to_arr_slice = arr_slice.as_mut_ptr();

    let slice_int: &mut [i32] = unsafe{
        slice::from_raw_parts_mut(row_pointer_to_arr_slice, 100)
    };

    let slice_int1: &mut [i32] = unsafe{
        slice::from_raw_parts_mut(row_pointer_to_arr_slice, 100)
    };

    for ele in slice_int {
        println!("ele is {}", ele);
    }

    for ele1 in slice_int1{ 
        println!("ele1 is {}", ele1);
    }
}

fn test4() {
    //  arr is owner
    let arr:[i32; 3] = [1, 2, 3];
    //
    let arr_slice: &[i32] = &arr;

    // 
    let row_ptr_to_arr_slice = arr_slice.as_ptr();

    let james: &[i32] = unsafe {
        slice::from_raw_parts(row_ptr_to_arr_slice, 100)
    };

    for i in james {
        println!("i is {}", i);
    }
    // 完全可以从一个immutable的引用中得到一个immutable的row pointer
    // 然后用 from_raw_parts 这个方法来获取对应的值。
    // 
}

/**
 * 直接声明一个 reference 呢？
 */
fn test5() {
    let slice_arr: &[i32] = &[1, 3];
    let row_ptr_to_slice_arr = slice_arr.as_ptr();
    let james: &[i32] = unsafe {
        slice::from_raw_parts(row_ptr_to_slice_arr, 100)
    };
    for jame in james {
        println!("jame is: {}", jame);
    }
    // 没毛病，直接声明一个reference 就可以了。
}

fn test6() {
    let arr: [i32; 3] = [1, 2, 3];
    let raw_ptr_arr = arr.as_ptr();
    // let james: [i32; 100] = unsafe{
    //     // *(slice::from_raw_parts(raw_ptr_arr, 100))
    // };
}