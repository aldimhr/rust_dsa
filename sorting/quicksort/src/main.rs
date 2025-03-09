/*
    Quicksort terbagi menajdi 3 bagian, pivot, left value dari pivot, dan right value dari pivot

    Partition
    -> Pivot, adalah angka acuan sebagai pembanding
    -> Proses ini akan membagi array menjadi 2 bagian
        -> jika nilai lebih kecil dari pivot maka geser ke kiri
        -> jika nilai lebih besar dari pivot maka geser ke kanan

    Quicksort
    -> Akan melakukan pemanggilan partitioning
    -> Hasil dari partitioning adalah index pivot selanjutnya
    -> Menggunakan recursive untuk melakukan partitioning
        -> Recursive untuk nilai bagian kiri pivot
        -> Recursive untuk nilai bagian kanan pivot
*/
fn main() {
    // println!("Hello, world!");
    println!("\n QUICKSORT HOARE");
    let mut arr: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let length = arr.len() - 1;
    quicksort_hoare(&mut arr, 0, length);

    println!("\n QUICKSORT LOMUTO");
    let mut arr: Vec<i32> = vec![-5, 3, -2, 0, 1];
    let length = arr.len() - 1;
    quicksort_lomuto(&mut arr, 0, length);
}

// hoare partition akan mengambil dari nilai tengah sebagai pivot
fn hoare_partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    println!("\narr before: {:?}", &arr);
    let pivot = arr[(lo + hi) / 2]; // ambil nilai tengah sebagai pivot

    println!("pivot: {}, lo: {}, hi: {}", &pivot, lo, hi);
    let mut i = lo;
    let mut j = hi; // index terakhir

    // pindahkan nilai < pivot ke kiri dan nilai > pivot ke kanan
    loop {
        while arr[i] < pivot {
            i += 1;
        }

        while arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            println!("arr after: {:?}", &arr);
            return j;
        }

        println!("arr swapped: {} {}", &arr[i], &arr[j]);

        arr.swap(i, j);

        println!("after swap: {:?}", &arr);
        i += 1;
        j -= 1;
    }
}

fn quicksort_hoare(arr: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo < hi {
        let pivot_index = hoare_partition(arr, lo, hi);

        if pivot_index > 0 {
            quicksort_hoare(arr, lo, pivot_index); // left
        }

        quicksort_hoare(arr, pivot_index + 1, hi); // right
    }
}

// lomuto partition akan mengambil nilai awal atau akhir sebagai pivot
fn lomuto_partition(arr: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    println!("\narr before: {:?}", &arr);
    let pivot = arr[hi]; // ambil nilai akhir sebagai pivot

    println!("pivot: {}, lo: {}, hi: {}", pivot, lo, hi);
    let mut i = lo;

    for j in lo..hi {
        if arr[j] <= pivot {
            println!("arr swapped: {} {}", arr[i], arr[j]);

            arr.swap(i, j);
            i += 1;

            println!("after swap: {:?}", &arr);
        }
    }

    arr.swap(i, hi); // Tukar posisi pivot dengan nilai tengah

    println!("arr after: {:?}", &arr);

    i
}

fn quicksort_lomuto(arr: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo < hi {
        let pivot_index = lomuto_partition(arr, lo, hi);

        if pivot_index > 0 {
            quicksort_lomuto(arr, lo, pivot_index - 1); // left
        }

        quicksort_lomuto(arr, pivot_index + 1, hi); // right
    }
}

#[cfg(test)]
mod lomuto_quicksort {
    use super::*;

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        let length = v.len() - 1;
        quicksort_lomuto(&mut v, 0, length);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut v = vec![1, 2, 3, 4, 5];
        let length = v.len() - 1;
        quicksort_lomuto(&mut v, 0, length);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        let length = v.len() - 1;
        quicksort_lomuto(&mut v, 0, length);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_numbers() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let length = v.len() - 1;
        quicksort_lomuto(&mut v, 0, length);
        assert_eq!(v, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut v = vec![-5, 3, -2, 0, 1];
        let length = v.len() - 1;
        quicksort_lomuto(&mut v, 0, length);
        assert_eq!(v, vec![-5, -2, 0, 1, 3]);
    }

    #[test]
    fn test_subarray_sorting() {
        let mut v = vec![9, 7, 5, 11, 12, 2, 14];
        // Urutkan hanya subarray indeks 2-5: [5, 11, 12, 2]
        quicksort_lomuto(&mut v, 2, 5);
        assert_eq!(v, vec![9, 7, 2, 5, 11, 12, 14]);
    }
}

#[cfg(test)]
mod hoare_quicksort {
    use super::*;

    #[test]
    fn test_single_element() {
        let mut v = vec![42];
        let length = v.len() - 1;
        quicksort_hoare(&mut v, 0, length);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_sorted_array() {
        let mut v = vec![1, 2, 3, 4, 5];
        let length = v.len() - 1;
        quicksort_hoare(&mut v, 0, length);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut v = vec![5, 4, 3, 2, 1];
        let length = v.len() - 1;
        quicksort_hoare(&mut v, 0, length);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_numbers() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let length = v.len() - 1;
        quicksort_hoare(&mut v, 0, length);
        assert_eq!(v, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut v = vec![-5, 3, -2, 0, 1];
        let length = v.len() - 1;
        quicksort_hoare(&mut v, 0, length);
        assert_eq!(v, vec![-5, -2, 0, 1, 3]);
    }

    #[test]
    fn test_subarray_sorting() {
        let mut v = vec![9, 7, 5, 11, 12, 2, 14];
        // Urutkan hanya subarray indeks 2-5: [5, 11, 12, 2]
        quicksort_hoare(&mut v, 2, 5);
        assert_eq!(v, vec![9, 7, 2, 5, 11, 12, 14]);
    }
}
