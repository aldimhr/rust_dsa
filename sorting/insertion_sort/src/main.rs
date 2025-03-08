fn main() {
    let mut arr = vec![1, 5, 3, 2];
    insertion_sort(&mut arr);
}

/*
    Insertion sort bekerja dengan cara menggeser data ke tempat sesuai dengan urutannya

    [1, 5, 3, 2]
    1. kita ambil angka 1 sebagai pembanding, lalu loop index sebelum 1, karena tidak ada
        maka lanjut loop
    2. kita ambil angka 5 sebagai pembanding, lalu loop index sebelum 5, ada angka 1, lalu cek
        apakah angka 5 < 1? karena false maka kita lanjutkan loop
    3. kita ambil angka 3 sebagai pembanding, lalu loop index sebelum 3, ada angka 5, lalu cek
        apakah 3 < 5? benar, maka swap
        kondisi sekarang array [1,3,5,2] setelah kita swap
    4. kita ambil angka 2 sebagai pembanding, lalu loop index sebelum 2, ada angka 5, lalu cek
        apakah 2 < 5? benar, maka swap
        kondisi sekarang array [1,3,2,5] setelah kita swap

        lanjut ke array sebelum 2, apakah 2 < 3? benar, maka swap
        kondisi sekarang array [1,2,3,5] setelah kita swap

        lanjut ke array sebelum 2, apakah 1 < 2? salah, maka end loop
*/
fn insertion_sort(arr: &mut Vec<i32>) {
    println!("arr before: {:?}", &arr);
    let length = arr.len();

    for i in 0..length {
        let mut value = arr[i];
        let mut index = i;

        // untuk menentukan tempat dari current value
        for j in (0..i).rev() {
            let before = arr[j];
            if value < before {
                arr.swap(index, j);

                index = j;
                value = arr[j];
            } else {
                break;
            }
        }
    }

    println!("arr after: {:?}", &arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut v = vec![];
        insertion_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        insertion_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_numbers() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut v = vec![-5, 3, -2, 0, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![-5, -2, 0, 1, 3]);
    }
}
