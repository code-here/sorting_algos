

fn main() {
    let mut arr = [ 32, 67, 45, 87, 11, 34, 66, 66, -66 ];
    println!("array before sorting: {:?}", &arr);
    
    
    //let arr = sorting_algos::bubble_sort(&mut arr);
    //let arr = sorting_algos::insertion_sort(&mut arr);
    let arr = sorting_algos::selection_sort(&mut arr);
    
    
    println!("array after sorting: {:?}", &arr);
}
