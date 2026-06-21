use super::tablero::{ALTO, ANCHO};
use rand::RngExt;

pub struct Proyectil {
    pub x: usize,
    pub y: usize,
    pub activo: bool,
}

pub struct Enemigo {
    pub x: usize,
    pub y: usize,
    pub activo: bool,
    pub tipo: u8,
    pub dir: i32,           // ← dirección vertical: 1 = abajo, -1 = arriba
    pub en_formacion: bool, // ← true = moviéndose en grupo, false = dive bomb
}

pub struct EstadoJuego {
    pub px: usize,
    pub py: usize,
    pub proyectiles: Vec<Proyectil>,
    pub enemigos: Vec<Enemigo>, // ← nuevo
    pub estrellas: Vec<(usize, usize)>,
    pub offset: usize,
}

impl EstadoJuego {
    pub fn new() -> Self {
        // Genera estrellas en posiciones aleatorias al inicio
        let mut estrellas = Vec::new();
        let mut rng = rand::rng(); // Initialize the RNG locally
        let mut rng = rand::rng();
        for _ in 0..40 {
            let x = rng.random_range(0..ANCHO);
            let y = rng.random_range(0..ALTO);
            estrellas.push((x, y));
        }
        let enemigos = vec![
            Enemigo {
                x: 18,
                y: 4,
                activo: true,
                tipo: 1,
                dir: 1,
                en_formacion: true,
            },
            Enemigo {
                x: 18,
                y: 10,
                activo: true,
                tipo: 1,
                dir: 1,
                en_formacion: true,
            },
            Enemigo {
                x: 18,
                y: 16,
                activo: true,
                tipo: 1,
                dir: 1,
                en_formacion: true,
            },
        ];

        EstadoJuego {
            px: 12,
            py: 12,
            proyectiles: Vec::new(),
            enemigos, // ← ahora sí tiene los enemigos
            estrellas,
            offset: 0,
        }
    }

    // Llama esto en el loop igual que actualizar_proyectiles()
    pub fn actualizar_estrellas(&mut self) {
        self.offset = (self.offset + 1) % ANCHO;
    }

    pub fn actualizar_enemigos(&mut self) {
        let px = self.px;
        let py = self.py;
        let mut rng = rand::rng();

        for enemigo in self.enemigos.iter_mut() {
            if !enemigo.activo {
                continue;
            }

            if enemigo.en_formacion {
                // 1% de probabilidad por tick de lanzarse
                if rng.random_range(0..100) == 0 {
                    enemigo.en_formacion = false;
                }

                let nueva_y = enemigo.y as i32 + enemigo.dir;
                if nueva_y <= 0 || nueva_y >= (ALTO as i32 - 3) {
                    enemigo.dir *= -1;
                }
                enemigo.y = (enemigo.y as i32 + enemigo.dir).clamp(0, ALTO as i32 - 3) as usize;
            } else {
                if enemigo.x == 0 {
                    enemigo.activo = false;
                } else {
                    enemigo.x -= 1;
                    if enemigo.y < py {
                        enemigo.y += 1;
                    } else if enemigo.y > py {
                        enemigo.y -= 1;
                    }
                }
            }
        }
    }

    pub fn galaga(
        tablero_estados: &mut [[u8; ANCHO]; ALTO],
        px: usize,
        py: usize,
        estado: &EstadoJuego,
    ) {
        //0 = vacio (.   ), 1 = jugador (@   ), 2 = proyectil (>   ), 3 = enemigo (#   ), 4 = proyectil_enemigo (<   )
        let enemigo_1 = ["033", "330", "033"];
        let enemigo_2 = ["00033", "00333", "03333", "00333", "000333"];

        let enemigo_1_explosion_1 = ["333", "333", "333"];
        let enemigo_1_explosion_2 = ["000", "030", "000"];

        let jugador = ["110", "011", "110"];
        let jugador_explosion_1 = ["111", "111", "111"];
        let jugador_explosion_2 = ["000", "010", "000"];

        let proyectil_1 = ["2"];
        let proyectil_2 = ["22", "22"];
        let proyectil_3 = ["200", "020", "200"];

        let proyectil_1_enemigo = ["4"];

        //Estampar al enemigo 1
        for enemigo in estado.enemigos.iter() {
            if !enemigo.activo {
                continue;
            }
            for (dy, fila) in enemigo_1.iter().enumerate() {
                for (dx, c) in fila.chars().enumerate() {
                    if c == '3' {
                        let ty = enemigo.y + dy;
                        let tx = enemigo.x + dx;
                        if ty < ALTO && tx < ANCHO {
                            tablero_estados[ty][tx] = 3;
                        }
                    }
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

    // Mueve todos los proyectiles activos hacia adelante
    pub fn actualizar_proyectiles(&mut self) {
        for p in self.proyectiles.iter_mut() {
            if p.activo {
                if p.x == 0 {
                    p.activo = false; // salió del tablero
                } else {
                    p.x += 1;
                }
            }
        }
        // Elimina proyectiles inactivos para no acumular memoria
        self.proyectiles.retain(|p| p.activo);
    }

    pub fn verificar_colisiones(&mut self) {
        let enemigo_1 = ["033", "330", "033"];

        for proyectil in self.proyectiles.iter_mut() {
            if !proyectil.activo {
                continue;
            }

            for enemigo in self.enemigos.iter_mut() {
                if !enemigo.activo {
                    continue;
                }

                // Revisar cada celda del sprite del enemigo
                let mut hubo_colision = false;
                'sprite: for (dy, fila) in enemigo_1.iter().enumerate() {
                    for (dx, c) in fila.chars().enumerate() {
                        if c == '3' {
                            let celda_x = enemigo.x + dx;
                            let celda_y = enemigo.y + dy;

                            if proyectil.x == celda_x && proyectil.y == celda_y {
                                hubo_colision = true;
                                break 'sprite;
                            }
                        }
                    }
                }

                if hubo_colision {
                    proyectil.activo = false;
                    enemigo.activo = false;
                }
            }
        }

        // Limpiar inactivos
        self.proyectiles.retain(|p| p.activo);
        self.enemigos.retain(|e| e.activo);
    }
}
