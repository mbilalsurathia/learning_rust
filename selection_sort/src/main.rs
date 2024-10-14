#![allow(dead_code)] //this line suppresses compiler warnings
fn main() {
    let mut arr = [ 
    222, 210,63, 75, 88,
    133, 20, 120, 107,
    99, 11, 47, 10, 155, 162,
    176,188, 199, 200, 2, 1,
    ];
   
    println!("Original array: {:?}", arr);
    
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}


fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
       
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}
