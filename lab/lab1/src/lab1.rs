fn fah_cel(fah: f32) -> f32{

    let mut cel: f32 = (fah - 32) * 5 / 9;
    
    cel
}

fn cel_fah(cel: f32) -> f32{

    let mut fah: f32 = cel * 9 / 5 + 32;

    fah
}
use std::io;

fn mian(){
    
    let mut m = String::new();
    let mut fah = String::new();
    let mut cel= String::new();

    println!("1--------computer fah -> cel");
    println!("2--------computer cel -> fah");
    
    io::stdin().read_line(&mut m)
        .expect("Failed to read line!");

    let m: u32 = m.trim()
                  .parse()
                  .expect("Please type a number!");

    if m == 1{
        println!("Input fahrenheit C:");
        
        io::stdin().read_line(&mut fah)
            .expect("Failed to read line!");

        let fah: f32 = fah.trim()
                      .parse()
                      .expect("Please type a number!");

        cel = fah_cel(fah);
        println!("Output Celsius: {} C", cel);
    }else if m == 2{
        println!("Input Celsius C:");
        io::stdin().read_line(&mut cel)
            .expect("Failed to read line!");

        let cel: f32 = fah.trim()
                      .parse()
                      .expect("Please type a number!");

        fah = fah_cel(fah);
        println!("Output Fahrenheit: {} C", fah);
    }
    else{

        println!("Input error!");
    }
}
