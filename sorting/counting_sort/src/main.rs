fn main() {
    let mut v = vec![2, 3, 0, 2, 3, 2];
    counting_sort(&mut v);
    println!("v: {:?}", &v);
}

/*
    Counting Sort

    Dilakukan dengan menghitung angka yang sama sesuai index.
    misalnya angka 2, maka dihitung di index 2, hingga berapa banyak data dengan angka 2 bisa dilihat
    di array penampung index 2

    [ 2, 3, 0, 2, 3, 2]
    1. buat array baru untuk menampung counter index based [0,0,0,0]
    2. pop angka terakhir [ 2, 3, 0, 2, 3] -> 2
        masukkan ke index 2: [0,0,1,0]
    3. pop angka terakhir [ 2, 3, 0, 2] -> 3
        masukkan ke index 3: [0,0,1,1]
    3. pop angka terakhir [ 2, 3, 0] -> 2
        masukkan ke index 2: [0,0,2,1]
    4. pop angka terakhir [ 2, 3] -> 0
        masukkan ke index 0: [1,0,2,1]
    5. pop angka terakhir [ 2] -> 3
        masukkan ke index 3: [1,0,2,2]
    6. pop angka terakhir [] -> 2
        masukkan ke index 2: [1,0,3,2]
    7. extract semua angka dari yang terkecil
        [1, 0, 3, 2] -> [0, 2, 2, 2, 3, 3]
*/
fn counting_sort(arr: &mut Vec<i32>) {
    let mut counter = vec![0; arr.len()];

    // counting number
    for i in 0..arr.len() {
        let value = arr[i] as usize;
        counter[value] += 1;
    }

    // replace arr
    let mut index_counter = 0;
    let mut index_arr = 0;
    loop {
        if index_arr >= counter.len() {
            break;
        }

        let curr_counter = counter[index_counter];
        if curr_counter <= 0 {
            index_counter += 1;
        } else {
            counter[index_counter] -= 1;
            arr[index_arr] = index_counter as i32;
            index_arr += 1;
        }
    }
}
