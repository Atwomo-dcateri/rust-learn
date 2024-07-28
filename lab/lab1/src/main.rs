fn fah_cel(fah: f32) -> f32 {
    let mut cel: f32 = (fah - 32.0) * 5.0 / 9.0;

    cel
}

fn cel_fah(cel: f32) -> f32 {
    let mut fah: f32 = cel * 9.0 / 5.0 + 32.0;

    fah
}
use std::io;

fn main() {
    let mut m = String::new();
    let mut fah = String::new();

    println!("1--------computer fah -> cel");
    println!("2--------computer cel -> fah");

    io::stdin().read_line(&mut m).expect("Failed to read line!");

    let m: u32 = m.trim().parse().expect("Please type a number!");

    if m == 1 {
        println!("Input fahrenheit C:");

        io::stdin()
            .read_line(&mut fah)
            .expect("Failed to read line!");

        let fah: f32 = fah.trim().parse().expect("Please type a number!");

        let mut cel1: f32 = fah_cel(fah);

        println!("Output Celsius: {} C", cel1);
    } else if m == 2 {
        let mut cel = String::new();

        println!("Input Celsius C:");

        io::stdin()
            .read_line(&mut cel)
            .expect("Failed to read line!");

        let cel: f32 = cel.trim().parse().expect("Please type a number!");

        let mut fah1: f32 = fah_cel(cel);
        println!("Output Fahrenheit: {} C", fah1);
    } else {
        println!("Input error!");
    }
}
