/*
    Array

    Array digunakan untuk menampung beberapa data. Setiap data dilengkapi dengan index yang menunjukkan
    dimana data tersebut berada. Rust menggunakan zero-based index, artinya index dimulai dari 0
    sebagai data pertama.

    Array, Vectors, and Slices

    Rust memiliki 3 tipe data yang memiliki sifat seperti Array pada umumnya.
    1. The type [T; N]
        Merepresentasikan array dengan N value dan tipe data T. Tipe ini bersifat konstant atau tidak
        berubah ubah. Kita tidak dapat menambahkan atau mengurangi element dari array ini.
    2. The type Vec<T>, atau disebut dengan vector dari T
        Tipe ini bersifat dinamis, kita bisa mengurangi atau menambahkan element didalamnya. Vector
        element berada di Heap Memory.
    3. The type &[T] and &mut [T], atau disebut dengan shared slice of T dan mutable slice of T
        TIpe ini merupakan referensi dari beberapa element yang merupakan bagian array atau vector.

    Katakanlah kita ingin mencari length dari ketiga tipe tersebut, bisa menggunakan v.len();
    Jika kita ingin mengambil value pertama cukup dengan v[i], i merupakan angka yang harus memiliki
    tipe data usize

    Array in rust

*/
fn main() {
    // ============ Array type [T; N]
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {:?}", a);

    // tipe ini bisa juga merepresentasikan beberapa data dengan value yang sama
    let a2: [i32; 6] = [1; 6]; // 1 sebanyak 6
    println!("a2: {:?}", a2);

    // ============ Array type Vec<T>
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(12);
    println!("v: {:?}", v);

    // dengan type vector ini kita juga bisa membuat beberapa data dengan value yang sama
    // menggunakan variabel
    let length = 10;
    let widht = 2;
    let v2: Vec<i32> = vec![0; length * widht];
    println!("v2: {:?}", v2);

    // kita bisa set vector empty
    let mut v_empty: Vec<i32> = Vec::new();
    v_empty.push(1);
    v_empty.push(2);
    println!("v_empty: {:?}", v_empty);

    // jika kita sudah tahu berapa value yang harus ada di vector, sebaiknya menggunakan
    // Vec::with_capacity() alih alih menggunakan Vec::new()
    let mut v_cap: Vec<i32> = Vec::with_capacity(3);
    v_cap.push(1);
    v_cap.push(2);
    v_cap.push(3);
    println!("v_cap: {:?}", v_cap);

    // dengan tipe vector kita bisa insert ataupun remove element didalamnya
    v_cap.insert(1, 10);
    println!("v_cap after insert(1, 10): {:?}", v_cap);

    v_cap.remove(3);
    println!("v_cap after remove(3): {:?}", v_cap);

    // kita juga bisa menghilangkan element terakhir menggunakan pop, ini akan mengembalikan
    // Some(value) atau None
    let v_pop = v_cap.pop();
    println!("v_pop: {:?}", v_pop);

    // ============ Array type [T] atau slice
    // tipe ini sangat umum digunakan sebagai tipe data parameter dari function
    fn sum(a: &[i32], b: &[i32]) -> i32 {
        a[0] + b[0]
    }

    // function sum dapat menerima vector ataupun fixed array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("sum {}", sum(&a, &b));

    // dengan slice kita bisa juga mengambil sebagian value dari array tipe lain
    let c = &a[3..]; // mengambil index ke 3 hingga akhir
    let d = &a[..3]; // mengambil index ke pertama hingga ke 3-1
    let e = &a[..=3]; // mengambil index ke pertama hingga ke 3
    let f = &a[2..3]; // mengambil data antara index 2 dan 3-1

    println!("c: {:?}", c);
    println!("d: {:?}", d);
    println!("e: {:?}", e);
    println!("f: {:?}", f);
}
