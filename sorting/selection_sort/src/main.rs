fn main() {
    let mut arr = vec![-5, 0, -3, 1, 4];
    println!("arr : {:?}", &arr);
    selection_sort(&mut arr);
    println!("result : {:?}", &arr);
}

/*
    SELECTION SORT

    Algoritma dengan menampung nilai paling kiri, lalu mencari nilai terkecil dan menukarnya.
    Mengurutkan nilai paling kecil ke besar dari kiri ke kanan

    Contoh:
    [1, 3, 2]

    1. Simpan nilai index 0, lalu bandingkan dengan index selanjutnya
    2. Karena masih lebih kecil index 0, lanjut ke index 2
    3. Karena masih lebih kecil index 0, maka pointer kiri akan berpindah ke index 1
    4. Membandingkan nilai index 1 dan index 2
    5. Swap, karena lebih kecil index 2
    ...
*/
fn selection_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;

        for j in i + 1..len {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![5]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 5, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![2, 3, 2, 1, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 1, -5, 4, 0];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![-5, -3, 0, 1, 4]);
    }

    #[test]
    fn test_all_identical() {
        let mut arr = vec![7, 7, 7, 7];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7]);
    }

    #[test]
    fn test_two_elements() {
        let mut arr1 = vec![3, 1];
        selection_sort(&mut arr1);
        assert_eq!(arr1, vec![1, 3]);

        let mut arr2 = vec![5, 5];
        selection_sort(&mut arr2);
        assert_eq!(arr2, vec![5, 5]);
    }
}
