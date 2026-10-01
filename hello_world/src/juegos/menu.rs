//! Menú principal de selección de juegos.
//!
//! Para añadir un juego nuevo al menú:
//! 1. Añadir una variante a [`OpcionMenu`].
//! 2. Añadir su etiqueta en `OpcionMenu::etiqueta`.
//! 3. Añadir el match en [`lanzador`] que construye e instancia el juego.
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, poll},
    execute, terminal,
};
use std::io::{self, Write};
use std::iter::repeat;
use std::time::Duration;

use crate::juegos::juego::ANCHO;

use super::conways::EstadoConway;
use super::galaga::EstadoJuego;
use super::juego::run;

/// Juegos disponibles en el menú.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpcionMenu {
    Galaga,
    Conway,
    Salir,
}

impl OpcionMenu {
    /// Lista ordenada de opciones que aparecen en el menú.
    pub fn todas() -> Vec<OpcionMenu> {
        vec![OpcionMenu::Galaga, OpcionMenu::Conway, OpcionMenu::Salir]
    }

    /// Texto que se muestra para cada opción.
    pub fn etiqueta(&self) -> &'static str {
        match self {
            OpcionMenu::Galaga => "Galaga",
            OpcionMenu::Conway => "Conway's Game of Life",
            OpcionMenu::Salir => "Salir",
        }
    }

    /// Descripción breve de cada opción (se muestra bajo la etiqueta).
    pub fn descripcion(&self) -> &'static str {
        match self {
            OpcionMenu::Galaga => "Defiende la nave de los enemigos.",
            OpcionMenu::Conway => "Simula células vivas. Edita y anima.",
            OpcionMenu::Salir => "Cierra el programa.",
        }
    }
}

/// Lanza el juego correspondiente a la opción elegida.
/// Devuelve `true` si el usuario quiere volver al menú, `false` si quiere salir.
pub fn lanzador(opcion: OpcionMenu) -> bool {
    match opcion {
        OpcionMenu::Galaga => {
            run(EstadoJuego::new());
            true
        }
        OpcionMenu::Conway => {
            run(EstadoConway::new());
            true
        }
        OpcionMenu::Salir => false,
    }
}

/// Muestra el menú en pantalla y devuelve la opción elegida.
///
/// Navegación:
/// - ↑/↓ moverse
/// - 1/2/3... seleccionar por número
/// - Enter confirmar
/// - Q salir
pub fn mostrar_menu() -> OpcionMenu {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide).unwrap();

    let opciones = OpcionMenu::todas();
    let mut seleccion = 0usize;
    let mensaje_1 = "SELECCIONA UN JUEGO";
    let ancho_linea = ANCHO * 4;

    loop {
        //Dibujar menú
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", repeat("=").take(ANCHO * 4).collect::<String>());

        println!("{:^width$}", mensaje_1, width = ancho_linea);

        println!("{}", repeat("=").take(ANCHO * 4).collect::<String>());
        println!();
        for (i, op) in opciones.iter().enumerate() {
            let texto = if i == seleccion {
                format!("  > [{}] {}  -  {}", i + 1, op.etiqueta(), op.descripcion())
            } else {
                format!("    [{}] {}  -  {}", i + 1, op.etiqueta(), op.descripcion())
            };
            // Aseguramos que la línea ocupe todo el ancho, alineada a la izquierda con relleno derecho
            println!("{:<width$}", texto, width = ancho_linea);
        }
        println!();
        println!();

        let instrucciones = "Flechas ↑↓ para navegar | Enter para seleccionar | Q salir";
        println!("{:^width$}", instrucciones, width = ancho_linea);
        println!("{}", repeat("=").take(ANCHO * 4).collect::<String>());
        stdout.flush().unwrap();

        //Leer input
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => {
                        if seleccion > 0 {
                            seleccion -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if seleccion < opciones.len() - 1 {
                            seleccion += 1;
                        }
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let n = c.to_digit(10).unwrap() as usize;
                        if n >= 1 && n <= opciones.len() {
                            seleccion = n - 1;
                        }
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().unwrap();
                        execute!(stdout, cursor::Show).unwrap();
                        return opciones[seleccion];
                    }
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().unwrap();
                        execute!(stdout, cursor::Show).unwrap();
                        return OpcionMenu::Salir;
                    }
                    _ => {}
                }
            }
        }
    }
}
