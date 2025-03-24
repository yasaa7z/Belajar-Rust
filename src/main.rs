// Ilmu baru

// Di rust tidak bisa membuat nama variabel dengan huruf besar, mungkin
// contoh "let akuSukaPisang"
// yang benar "let aku_suka_pisang"

use std::io;

fn main() {
    println!("Masukan bilangan :");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Gagal menerima input");

    let bilangan_user: i32 = input.trim().parse()
        .expect("Input bukan bilangan bos");

    if bilangan_user % 2 == 0 {
        println!("Bilangan {} adalah genap!", bilangan_user);
    } else {
        println!("Bilangan {} adalahhh ganjil!", bilangan_user);
    }
}