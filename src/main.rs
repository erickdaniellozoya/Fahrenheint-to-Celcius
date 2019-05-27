use std::io;

fn main() {
    loop{
        println!("Convertidor de temperatura");
        println!("¿Qué quiere hacer?");
        println!("1. Convertir Fahrenheit a Celsius");
        println!("2. Convertir Celsius a Fahrenheit");
        println!("3. Salir");
        println!("Elige una opción: ");
        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line");
        let option: u8 = option.trim().parse()
            .expect("Is not a number");
        if option == 1 {
            println!("Grados Fahrenheint: ");
            let mut fah = String::new();
            io::stdin().read_line(&mut fah)
                .expect("Failed!");
            let fah: f64 = fah.trim().parse()
                .expect("Failed!");
            println!("{}° Fahrenheit equivalen a {}° Celsius", fah, fah_to_cel(fah));
        } else if option == 2 {
            println!("Grados Celsius: ");
            let mut cel = String::new();
            io::stdin().read_line(&mut cel)
                .expect("Failed!");
            let cel: f64 = cel.trim().parse()
                .expect("Failed!");
            println!("{}° Celsius equivalen a {}° Fahrenheit", cel, cel_to_fah(cel));
        }else if option == 3 {
            break;
        }else if option != 1 && option != 2 && option != 3{
            continue;
        }
    }
}

fn fah_to_cel(fah: f64) -> f64 {
    (fah - 32.0) * 5.0 / 9.0
}

fn cel_to_fah(cel: f64) -> f64 {
    (cel * 9.0 / 5.0) + 32.0
}