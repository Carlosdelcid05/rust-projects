use super::tablero::{ALTO, ANCHO};

pub struct Proyectil {
    pub x: usize,
    pub y: usize,
    pub activo: bool,
}

pub struct EstadoJuego {
    pub px: usize,
    pub py: usize,
    pub proyectiles: Vec<Proyectil>,
}

impl EstadoJuego {
    pub fn new() -> Self {
        EstadoJuego {
            px: 12,
            py: 12,
            proyectiles: Vec::new(),
        }
    }

    pub fn galaga(
        tablero_estados: &mut [[u8; ANCHO]; ALTO],
        px: usize,
        py: usize,
        estado: &EstadoJuego,
    ) {
        //0 = vacio, 1 = jugador, 2 = proyectil, 3 = enemigo
        let enemigo_1 = ["033", "330", "033"];
        let enemigo_2 = ["00033", "00333", "03333", "00333", "000333"];

        let jugador = ["110", "011", "110"];

        let proyectil_1 = ["2"];
        let proyectil_2 = ["22", "22"];
        let proyectil_3 = ["200", "020", "200"];

        //Estampar al enemigo 1
        for (dy, fila) in enemigo_1.iter().enumerate() {
            for (dx, c) in fila.chars().enumerate() {
                if c == '3' {
                    tablero_estados[dy + 2][dx + 2] = 3;
                }
            }
        }

        // Estampar jugador centrado en (px, py)
        let sprite_alto = jugador.len();
        let sprite_ancho = jugador[0].len();
        let origen_y = py.saturating_sub(sprite_alto / 2);
        let origen_x = px.saturating_sub(sprite_ancho / 2);

        for (dy, fila) in jugador.iter().enumerate() {
            for (dx, c) in fila.chars().enumerate() {
                let ty = origen_y + dy;
                let tx = origen_x + dx;
                // Verificar que no salgamos del tablero
                if ty < ALTO && tx < ANCHO && c == '1' {
                    tablero_estados[ty][tx] = 1;
                }
            }
        }

        // Estampar proyectiles
        for p in estado.proyectiles.iter() {
            for (dy, fila) in proyectil_1.iter().enumerate() {
                for (dx, c) in fila.chars().enumerate() {
                    let ty = p.y + dy;
                    let tx = p.x + dx;
                    if ty < ALTO && tx < ANCHO && c == '2' {
                        tablero_estados[ty][tx] = 2;
                    }
                }
            }
        }
    }

    // Crea un proyectil en la posición actual del jugador
    pub fn disparar(&mut self) {
        self.proyectiles.push(Proyectil {
            x: self.px + 2,
            y: self.py,
            activo: true,
        });
    }

    // Mueve todos los proyectiles activos hacia arriba
    pub fn actualizar_proyectiles(&mut self) {
        for p in self.proyectiles.iter_mut() {
            if p.activo {
                if p.y == 0 {
                    p.activo = false; // salió del tablero
                } else {
                    p.x += 1;
                }
            }
        }
        // Elimina proyectiles inactivos para no acumular memoria
        self.proyectiles.retain(|p| p.activo);
    }
}
