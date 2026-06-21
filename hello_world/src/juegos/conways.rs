use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn definir_plano() {
    let mut matriz: [[u8; 27]; 27] = [[0u8; 27]; 27];

    let pulsar = [
        "000010000010000",
        "000010000010000",
        "000011000110000",
        "000000000000000",
        "111001101100111",
        "001010101010100",
        "000011000110000",
        "000000000000000",
        "000011000110000",
        "001010101010100",
        "111001101100111",
        "000000000000000",
        "000011000110000",
        "000010000010000",
        "000010000010000",
    ];
    for (y, fila) in pulsar.iter().enumerate() {
        for (x, c) in fila.chars().enumerate() {
            if c == '1' {
                matriz[y + 5][x + 5] = 1;
            }
        }
    }

    for x in 1..=3 {
        matriz[2][x + 1] = 1;
    }
    for x in 21..=23 {
        matriz[2][x + 1] = 1;
    }
    for x in 1..=3 {
        matriz[23][x + 1] = 1;
    }
    for x in 21..=23 {
        matriz[23][x + 1] = 1;
    }

    for y in 11..=13 {
        matriz[y + 1][0] = 1;
    }
    for y in 11..=13 {
        matriz[y + 1][24] = 1;
    }

    for x in 11..=13 {
        matriz[2][x + 1] = 1;
    }
    for x in 11..=13 {
        matriz[24][x + 1] = 1;
    }

    matriz[3][0] = 1;
    matriz[3][1] = 1;
    matriz[4][0] = 1;
    matriz[4][1] = 1;

    matriz[3][23] = 1;
    matriz[3][24] = 1;
    matriz[4][23] = 1;
    matriz[4][24] = 1;

    matriz[20][0] = 1;
    matriz[20][1] = 1;
    matriz[21][0] = 1;
    matriz[21][1] = 1;

    matriz[20][23] = 1;
    matriz[20][24] = 1;
    matriz[21][23] = 1;
    matriz[21][24] = 1;

    mostrar_plano(&matriz);

    while tiene_vivos(&matriz) {
        thread::sleep(Duration::from_millis(200));
        conways_life(&mut matriz);
    }
}

fn tiene_vivos(matriz: &[[u8; 27]; 27]) -> bool {
    for fila in matriz {
        for &valor in fila {
            if valor == 1 {
                return true;
            }
        }
    }
    false
}

fn mostrar_plano(matriz: &[[u8; 27]; 27]) {
    print!("\x1B[2J");
    print!("\x1B[0;0H");
    std::io::stdout().flush().unwrap();

    for fila in matriz {
        for valor in fila {
            if *valor == 1 {
                print!("#  ");
            } else {
                print!(".  ");
            }
        }
        println!();
    }
    println!("===========================================================================");
    std::io::stdout().flush().unwrap();
}
fn conways_life(matriz: &mut [[u8; 27]; 27]) {
    let mut matriz_contador: [[u8; 27]; 27] = [[0u8; 27]; 27];

    for (y, fila) in matriz.iter().enumerate() {
        for (x, _valor) in fila.iter().enumerate() {
            let vecinos = [
                (y.wrapping_sub(1), x.wrapping_sub(1)),
                (y.wrapping_sub(1), x),
                (y.wrapping_sub(1), x + 1),
                (y, x.wrapping_sub(1)),
                (y, x + 1),
                (y + 1, x.wrapping_sub(1)),
                (y + 1, x),
                (y + 1, x + 1),
            ];

            for (vy, vx) in vecinos {
                if let Some(1) = matriz.get(vy).and_then(|f| f.get(vx)) {
                    matriz_contador[y][x] += 1;
                }
            }
        }
    }

    for (y, fila) in matriz_contador.iter().enumerate() {
        for (x, &conteo) in fila.iter().enumerate() {
            if matriz[y][x] == 1 {
                if !(conteo == 2 || conteo == 3) {
                    matriz[y][x] = 0;
                }
            } else {
                if conteo == 3 {
                    matriz[y][x] = 1;
                }
            }
        }
    }

    mostrar_plano(matriz);
}
