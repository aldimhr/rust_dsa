fn main() {
    let mut a = vec![2, 1, 4, 3];
    println!("before : {:?}", &a);
    bubble_sort(&mut a);
    println!("result : {:?}", &a);
}

/*
    BUBBLE SORT
    Konsep algo ini yaitu dengan cara menukar antara 2 nilai, lalu menggesert nilainya hingga
    berurutan.

    Contohnya, sorting ascending - dari kecil ke besar
    ==> [ 2, 1, 4, 3 ]
    1. bandingkan angka 2 dan 1 -> 1 lebih kecil
    2. swap -> pindah posisi 1 menjadi ke kiri
    ==> [ 1, 2, 4, 3 ]
    3. bandingkan 2 dan 4 -> 2 berada dikiri, artinya sudah sesuai, tidak perlu dilakukan swap
    4. bandingkan 4 dan 3 -> 3 lebih kecil
    5. swap -> pindahkan 3 ke posisi 4
    ==> [ 1, 2, 3, 4 ]
*/
fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    for i in 0..len {
        for j in 1..len - i {
            let curr = arr[j];
            let prev = arr[j - 1];

            // check is prev > curr?
            if prev > curr {
                // arr.swap(i, i - 1);

                // swap
                let temp = curr;
                arr[j] = prev;
                arr[j - 1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr: Vec<i32> = vec![5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![5]);
    }

    #[test]
    fn test_sorted() {
        let mut arr: Vec<i32> = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr: Vec<i32> = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr: Vec<i32> = vec![3, 1, 2, 3, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr: Vec<i32> = vec![-5, 0, 3, -2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![-5, -2, 0, 1, 3]);
    }

    #[test]
    fn test_random() {
        let mut arr: Vec<i32> = vec![5, 3, 7, 1, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7]);
    }

    #[test]
    fn test_two_elements() {
        let mut arr: Vec<i32> = vec![3, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 3]);
    }

    #[test]
    fn test_all_identical() {
        let mut arr: Vec<i32> = vec![2, 2, 2, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![2, 2, 2, 2]);
    }
}
