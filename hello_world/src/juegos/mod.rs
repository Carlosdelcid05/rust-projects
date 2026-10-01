//! Módulo raíz de los juegos.
//!
//! Para añadir un juego nuevo:
//! 1. Crea el archivo `mi_juego.rs` dentro de esta carpeta.
//! 2. Decláralo aquí con `pub mod mi_juego;`.
//! 3. Implementa el trait [`Juego`](crate::juegos::juego::Juego) en su estado.
//! 4. Añade una variante a `OpcionMenu` en `menu.rs` y regístrala en `lanzador`.

pub mod conways;
pub mod galaga;
pub mod juego;
pub mod menu;
pub mod tablero;
