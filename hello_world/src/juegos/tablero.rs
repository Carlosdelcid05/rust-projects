//! Módulo de compatibilidad hacia atrás.
//!
//! Históricamente este archivo contenía el bucle principal del juego Galaga
//! junto con las constantes `ANCHO`/`ALTO`. Toda esa lógica se ha movido a
//! `juego.rs` (bucle genérico) y `galaga.rs` (estado del juego).
//!
//! Reexportamos las constantes para que cualquier código externo que siga
//! haciendo `use juegos::tablero::{ANCHO, ALTO}` siga compilando, y
//! mantenemos `tablero()` como atajo para ejecutar Galaga directamente.

use super::galaga::EstadoJuego;
use super::juego::run;

// Reexportación de constantes compartidas (compatibilidad hacia atrás).
#[allow(unused_imports)]
pub use super::juego::{ALTO, ANCHO};

/// Atajo de compatibilidad: ejecuta Galaga directamente (sin pasar por el menú).
/// Equivale a `juegos::juego::run(EstadoJuego::new())`.
pub fn tablero() {
    run(EstadoJuego::new());
}
