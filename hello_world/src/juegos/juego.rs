//! Infraestructura común para todos los juegos.
//!
//! Este módulo define:
//! - Las constantes `ANCHO` y `ALTO` del tablero estándar.
//! - El trait [`Juego`] que cualquier juego debe implementar.
//! - La función [`run`] que ejecuta el bucle principal de forma genérica.
//!
//! Para añadir un juego nuevo basta con:
//! 1. Crear un módulo dentro de `juegos/`.
//! 2. Implementar el trait `Juego` para el estado del juego.
//! 3. Registrarlo en el menú (`menu.rs`).

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, poll},
    execute, terminal,
};
use std::io;
use std::time::Duration;

/// Ancho estándar del tablero. Compartido por todos los juegos.
pub const ANCHO: usize = 47;
/// Alto estándar del tablero. Compartido por todos los juegos.
pub const ALTO: usize = 24;

/// Trait común para todos los juegos.
///
/// Cada juego es responsable de su propio estado interno; el bucle
/// principal (definido en [`run`]) se encarga únicamente de leer input,
/// llamar a `actualizar`, renderizar y comprobar si el juego sigue activo.
pub trait Juego {
    /// Nombre del juego (se muestra en menús y cabeceras).
    fn nombre(&self) -> &str;

    /// Instrucciones breves que se muestran en el pie del tablero.
    fn instrucciones(&self) -> &str;

    /// Procesa una tecla presionada por el usuario.
    fn procesar_input(&mut self, key: KeyCode);

    /// Avanza la lógica del juego un tick.
    /// `tick` es un contador creciente (con wrapping) que cada juego
    /// puede usar para cadenciar sus actualizaciones (e.g. `tick % 6 == 0`).
    fn actualizar(&mut self, tick: u32);

    /// Dibuja el estado actual en la terminal.
    /// El bucle principal ya posiciona el cursor en (0,0) antes de llamar;
    /// el juego puede limpiar la pantalla o simplemente sobreescribir.
    fn renderizar(&self, stdout: &mut io::Stdout);

    /// Devuelve `false` cuando el juego debe terminar (típicamente porque
    /// el usuario presionó 'Q'). El bucle principal sale en ese caso.
    fn esta_activo(&self) -> bool;
}

/// Bucle principal genérico.
///
/// Activa modo raw, oculta el cursor, y ejecuta el juego a ~30 FPS
/// hasta que `esta_activo()` retorne `false`. Al terminar restaura la
/// terminal a su estado normal.
pub fn run<J: Juego>(mut juego: J) {
    terminal::enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Hide).unwrap();

    let mut tick: u32 = 0;

    loop {
        //Leer input SIN bloquear
        if poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                juego.procesar_input(key.code);
                if !juego.esta_activo() {
                    break;
                }
            }
        }

        tick = tick.wrapping_add(1);
        juego.actualizar(tick);

        // Renderizar
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        juego.renderizar(&mut stdout);

        //Controlar FPS (~30)
        std::thread::sleep(Duration::from_millis(1000 / 30));
    }

    // Restaurar terminal al salir
    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();
}
