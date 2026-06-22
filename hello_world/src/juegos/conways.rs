//! Conway's Game of Life — versión interactiva.
//!
//! Controles:
//! - **Flechas**: mover el cursor por el tablero.
//! - **C**: colocar una célula viva (`#`) en la posición del cursor.
//! - **X**: eliminar la célula en la posición del cursor.
//! - **V**: alternar entre **modo Edición** y **modo Animación**.
//!           - En modo Edición, el usuario pinta células manualmente y la
//!             simulación está pausada.
//!           - En modo Animación, las reglas de Conway se aplican
//!             automáticamente (~5 generaciones/seg).
//! - **Q**: salir al menú principal.
//!
//! Nota: el cursor puede moverse libremente en ambos modos, pero las
//! teclas `C` y `X` solo tienen efecto en modo Edición.

use super::juego::{ALTO, ANCHO, Juego};
use crossterm::event::KeyCode;
use std::io::{self, Write};

/// Modo de operación del juego.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModoConway {
    /// El usuario puede pintar y borrar células; la simulación está pausada.
    Edicion,
    /// La simulación corre automáticamente generación a generación.
    Animacion,
}

impl ModoConway {
    fn etiqueta(&self) -> &'static str {
        match self {
            ModoConway::Edicion => "EDIT",
            ModoConway::Animacion => "ANIM",
        }
    }
}

pub struct EstadoConway {
    /// Matriz de células: `1` = viva, `0` = muerta.
    pub matriz: [[u8; ANCHO]; ALTO],
    /// Posición X del cursor (columna).
    pub cursor_x: usize,
    /// Posición Y del cursor (fila).
    pub cursor_y: usize,
    /// Modo actual del juego.
    pub modo: ModoConway,
    /// `true` cuando el usuario ha presionado 'Q' y quiere salir.
    pub salir: bool,
    /// Contador de generaciones transcurridas (solo avanza en modo Animación).
    pub generacion: u32,
}

impl EstadoConway {
    pub fn new() -> Self {
        EstadoConway {
            matriz: [[0u8; ANCHO]; ALTO],
            cursor_x: ANCHO / 2,
            cursor_y: ALTO / 2,
            modo: ModoConway::Edicion,
            salir: false,
            generacion: 0,
        }
    }

    /// Cuenta los vecinos vivos de la celda `(x, y)` usando vecindad de
    /// Moore (8 vecinos) con bordes finitos (sin wrap-around).
    fn contar_vecinos(&self, x: usize, y: usize) -> u8 {
        let mut contador = 0u8;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if ny >= 0 && ny < ALTO as i32 && nx >= 0 && nx < ANCHO as i32 {
                    if self.matriz[ny as usize][nx as usize] == 1 {
                        contador += 1;
                    }
                }
            }
        }
        contador
    }

    /// Aplica una generación de las reglas de Conway:
    /// - Célula viva con 2 o 3 vecinos sobrevive.
    /// - Célula muerta con exactamente 3 vecinos nace.
    /// - Resto de casos: muere o permanece muerta.
    fn paso_simulacion(&mut self) {
        // Primero calculamos el conteo de vecinos para todas las celdas,
        // luego aplicamos las reglas simultáneamente.
        let mut vecinos: [[u8; ANCHO]; ALTO] = [[0u8; ANCHO]; ALTO];
        for y in 0..ALTO {
            for x in 0..ANCHO {
                vecinos[y][x] = self.contar_vecinos(x, y);
            }
        }
        for y in 0..ALTO {
            for x in 0..ANCHO {
                let v = vecinos[y][x];
                if self.matriz[y][x] == 1 {
                    if !(v == 2 || v == 3) {
                        self.matriz[y][x] = 0;
                    }
                } else if v == 3 {
                    self.matriz[y][x] = 1;
                }
            }
        }
        self.generacion = self.generacion.saturating_add(1);
    }
}

// ──────────────────────────────────────────────────────────────────────
//  Implementación del trait Juego.
// ──────────────────────────────────────────────────────────────────────
impl Juego for EstadoConway {
    fn nombre(&self) -> &str {
        "Conway's Game of Life"
    }

    fn instrucciones(&self) -> &str {
        " C colocar | X eliminar | V animar/pausar | Q salir"
    }

    fn procesar_input(&mut self, key: KeyCode) {
        match key {
            // ── Salir ────────────────────────────────────────────
            KeyCode::Char('q') => self.salir = true,

            // ── Alternar modo Edición ↔ Animación ────────────────
            KeyCode::Char('v') => {
                self.modo = match self.modo {
                    ModoConway::Edicion => ModoConway::Animacion,
                    ModoConway::Animacion => ModoConway::Edicion,
                };
            }

            // ── Colocar célula (solo en Edición) ─────────────────
            KeyCode::Char('c') => {
                if self.modo == ModoConway::Edicion {
                    self.matriz[self.cursor_y][self.cursor_x] = 1;
                }
            }

            // ── Eliminar célula (solo en Edición) ────────────────
            KeyCode::Char('x') => {
                if self.modo == ModoConway::Edicion {
                    self.matriz[self.cursor_y][self.cursor_x] = 0;
                }
            }

            // ── Movimiento del cursor (válido en ambos modos) ────
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < ALTO - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x < ANCHO - 1 {
                    self.cursor_x += 1;
                }
            }

            _ => {}
        }
    }

    fn actualizar(&mut self, tick: u32) {
        // En modo Animación, avanzamos una generación cada 6 ticks (~200ms
        // a 30 FPS => ~5 generaciones/segundo).
        if self.modo == ModoConway::Animacion && tick % 6 == 0 {
            self.paso_simulacion();
        }
    }

    fn renderizar(&self, stdout: &mut io::Stdout) {
        print!("\x1B[2J\x1B[1;1H");

        println!();

        for y in 0..ALTO {
            for x in 0..ANCHO {
                let viva = self.matriz[y][x] == 1;
                if x == self.cursor_x && y == self.cursor_y {
                    // Celda bajo el cursor: usamos corchetes para destacarla.
                    if viva {
                        print!("[#]");
                    } else {
                        print!("[ ]");
                    }
                } else if viva {
                    print!(" # ");
                } else {
                    print!(" . ");
                }
            }
            println!();
        }
        println!();
        print!(" {}", self.instrucciones());
        println!(" Modo: {:}|Gen: {}", self.modo.etiqueta(), self.generacion);
        stdout.flush().unwrap();
    }

    fn esta_activo(&self) -> bool {
        !self.salir
    }
}
