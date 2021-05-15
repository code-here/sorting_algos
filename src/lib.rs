pub fn bubble_sort(arr: &mut [i32]) -> &mut [i32] {
    heading("using Bubble Sort:");
    let length = arr.len() - 1;
    for j in 0..length {
        for i in 0..(length - j) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                println!("swapped {} and {}: {:?}", arr[i + 1], arr[i], arr);
            }
        }
    }
    arr
}

pub fn i_bubble_sort(arr: &mut [i32]) -> &mut [i32] {
    heading(" using improved bubble sort:");
    let length = arr.len() - 1;
    let mut sorted = true;
    for j in 0..length {
        for i in 0..(length - j) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                println!("swapped {} and {}: {:?}", arr[i + 1], arr[i], arr);
                sorted = false;
            }
        }
        if sorted {
            return arr;
        }
    }
    arr
}

pub fn insertion_sort(arr: &mut [i32]) -> &mut [i32] {
    heading("using insertion sort:");
    for i in 0..arr.len() {
        let temp = arr[i];
        let mut has_moved = false;
        let mut not_placed = true;
        for j in (0..i).rev() {
            if arr[j] > temp {
                arr[j + 1] = arr[j];
                has_moved = true;
            } else {
                arr[j + 1] = temp;
                not_placed = false;
                break;
            }
        }
        if has_moved && not_placed {
            arr[0] = temp;
        }
    }
    arr
}

pub fn selection_sort(arr: &mut [i32]) -> &mut [i32] {
    heading("using selection sort:");
    let mut length = arr.len();
    for _ in 0..arr.len() {
        let mut temp_index = 0;
        for j in 0..length {
            if arr[temp_index] < arr[j] {
                temp_index = j;
            }
            if j == length - 1 {
                arr.swap(j, temp_index);
            }
        }
        length -= 1;
    }
    arr
}

fn heading(title: &str) {
    println!("\x1b[38;5;45m{}\x1b[0m", title);
}

#[cfg(test)]
mod tests;
