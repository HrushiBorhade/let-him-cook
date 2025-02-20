pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;
        for j in 1..(n - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut arr = [64, 34, 25, 12, 22, 11, 90];
        println!("Original array: {:?}", arr);

        bubble_sort(&mut arr);

        println!("Sorted array: {:?}", arr);

        // Assert that the sorted array matches the expected result
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test2() {
        let mut vec = vec![1, 5, 10, 7, 22];
        println!("Original vec: {:?}", vec);

        bubble_sort(&mut vec);

        println!("Sorted vec: {:?}", vec);

        // Assert that the sorted vector matches the expected result
        assert_eq!(vec, vec![1, 5, 7, 10, 22]);
    }
}
