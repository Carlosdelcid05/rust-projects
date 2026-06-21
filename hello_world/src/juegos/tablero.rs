use super::galaga::EstadoJuego;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, poll},
    execute, terminal,
};
use std::io::{self, Write};
use std::sync::Mutex;
use std::time::Duration;

pub const ANCHO: usize = 24;
pub const ALTO: usize = 24;

pub static TABLERO_ESTADOS: Mutex<[[u8; ANCHO]; ALTO]> = Mutex::new([[0u8; ANCHO]; ALTO]);

pub fn tablero() {
    let tablero_estados: &mut [[u8; ANCHO]; ALTO] = &mut [[0u8; ANCHO]; ALTO];
    let mut estado = EstadoJuego::new();
    EstadoJuego::galaga(tablero_estados, 0, 0, &estado);
    // Activa modo raw — la terminal deja de procesar input automáticamente
    terminal::enable_raw_mode().unwrap();

    let mut stdout = io::stdout();

    // Oculta el cursor para que no parpadee
    execute!(stdout, cursor::Hide).unwrap();

    let mut x: usize = 12;
    let mut y: usize = 12;

    // Contador para controlar velocidad del proyectil
    let mut tick: u32 = 0;

    loop {
        // ── Leer input SIN bloquear ──────────────────────────
        // poll() pregunta "¿hay tecla presionada?" y retorna inmediatamente
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => break,             // salir
                    KeyCode::Char('c') => estado.disparar(), // ← DISPARAR
                    KeyCode::Char('v') => {}                 // boton de acción 2
                    KeyCode::Char('x') => {}                 // boton de acción 3
                    KeyCode::Up => estado.py = estado.py.saturating_sub(1),
                    KeyCode::Down => {
                        if estado.py < ALTO - 1 {
                            estado.py += 1;
                        }
                    }
                    KeyCode::Left => estado.px = estado.px.saturating_sub(1),
                    KeyCode::Right => {
                        if estado.px < ANCHO - 1 {
                            estado.px += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        tick += 1;
        if tick % 3 == 0 {
            estado.actualizar_proyectiles();
        }
        // Limpiar estado del frame anterior
        *tablero_estados = [[0u8; ANCHO]; ALTO];

        // Estampar todos los objetos con sus posiciones actuales
        EstadoJuego::galaga(tablero_estados, estado.px, estado.py, &estado);

        // Dibujar
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        dibujar(&mut stdout, tablero_estados);

        // ── Controlar FPS ────────────────────────────────────
        std::thread::sleep(Duration::from_millis(1000 / 30));
    }

    // ── Restaurar terminal al salir ──────────────────────────
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();
}

fn dibujar(stdout: &mut io::Stdout, tablero_estados: &[[u8; ANCHO]; ALTO]) {
    let tablero = *tablero_estados; // ya tiene todo estampado

    print!("\x1B[2J\x1B[1;1H");
    for y in 0..ALTO {
        for x in 0..ANCHO {
            match tablero[y][x] {
                1 => print!("@   "),
                2 => print!(">   "),
                3 => print!("#   "),
                _ => print!(".   "),
            }
        }
        println!();
    }
    print!("Muevete con flechas, Q para salir");
    stdout.flush().unwrap();
}
