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
    pub dir: i32,
    pub en_formacion: bool,
    pub explosion_tick: u8,
}

pub struct EstadoJuego {
    pub px: usize,
    pub py: usize,
    pub proyectiles: Vec<Proyectil>,
    pub enemigos: Vec<Enemigo>,
    pub estrellas: Vec<(usize, usize)>,
    pub offset: usize,
    pub vidas: u8,
    pub jugador_activo: bool,
    pub invulnerable_ticks: u32,
    pub explosion_jugador_tick: u8,
}

impl EstadoJuego {
    pub fn new() -> Self {
        // Genera estrellas en posiciones aleatorias al inicio
        let mut estrellas = Vec::new();

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
                explosion_tick: 0,
            },
            Enemigo {
                x: 18,
                y: 10,
                activo: true,
                tipo: 1,
                dir: 1,
                en_formacion: true,
                explosion_tick: 0,
            },
            Enemigo {
                x: 18,
                y: 16,
                activo: true,
                tipo: 1,
                dir: 1,
                en_formacion: true,
                explosion_tick: 0,
            },
        ];

        EstadoJuego {
            px: 12,
            py: 12,
            proyectiles: Vec::new(),
            enemigos,
            estrellas,
            offset: 0,
            vidas: 3,
            jugador_activo: true,
            invulnerable_ticks: 0,
            explosion_jugador_tick: 0,
        }
    }

    pub fn verificar_colision_jugador(&mut self) {
        if self.invulnerable_ticks > 0 {
            self.invulnerable_ticks = self.invulnerable_ticks.saturating_sub(1);
            return;
        }
        if !self.jugador_activo || self.explosion_jugador_tick > 0 {
            return;
        }

        let jugador = ["110", "011", "110"];
        let enemigo_1 = ["033", "330", "033"];

        let celdas_jugador = EstadoJuego::celdas_sprite(&jugador, self.px, self.py, '1');

        for enemigo in self.enemigos.iter_mut() {
            if !enemigo.activo || enemigo.explosion_tick > 0 {
                continue;
            }

            for (dy, fila) in enemigo_1.iter().enumerate() {
                for (dx, c) in fila.chars().enumerate() {
                    if c != '3' {
                        continue;
                    }
                    let ey = enemigo.y + dy;
                    let ex = enemigo.x + dx;

                    if celdas_jugador.contains(&(ey, ex)) {
                        enemigo.explosion_tick = 1;
                        self.explosion_jugador_tick = 1;
                        return;
                    }
                }
            }
        }
    }
    pub fn actualizar_explosiones(&mut self) {
        // 1. Actualizar enemigos
        for enemigo in self.enemigos.iter_mut() {
            if enemigo.explosion_tick > 0 {
                enemigo.explosion_tick += 1;
                // Si llega a 4 ticks (2 frames de animación), se elimina
                if enemigo.explosion_tick > 4 {
                    enemigo.activo = false;
                }
            }
        }
        self.enemigos.retain(|e| e.activo);

        // 2. Actualizar jugador
        if self.explosion_jugador_tick > 0 {
            self.explosion_jugador_tick += 1;

            if self.explosion_jugador_tick > 4 {
                self.explosion_jugador_tick = 0; // Termina animación

                // Aplicar penalización ahora
                self.vidas = self.vidas.saturating_sub(1);
                if self.vidas == 0 {
                    self.jugador_activo = false; // GAME OVER
                } else {
                    // Respawn
                    self.px = ANCHO / 2;
                    self.py = ALTO / 2;
                    self.invulnerable_ticks = 30;
                }
            }
        }
    }

    fn celdas_sprite(sprite: &[&str], cx: usize, cy: usize, caracter: char) -> Vec<(usize, usize)> {
        let alto = sprite.len();
        let ancho = sprite[0].len();
        let origen_y = cy.saturating_sub(alto / 2);
        let origen_x = cx.saturating_sub(ancho / 2);

        let mut celdas = Vec::new();
        for (dy, fila) in sprite.iter().enumerate() {
            for (dx, c) in fila.chars().enumerate() {
                if c == caracter {
                    let ty = origen_y + dy;
                    let tx = origen_x + dx;
                    if ty < ALTO && tx < ANCHO {
                        celdas.push((ty, tx));
                    }
                }
            }
        }
        celdas
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
            if enemigo.explosion_tick > 0 {
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

        // Estampar enemigos (normales o explosión)
        let enemigo_1 = ["033", "330", "033"];
        let enemigo_1_explosion_1 = ["333", "333", "333"];
        let enemigo_1_explosion_2 = ["000", "030", "000"];

        for enemigo in estado.enemigos.iter() {
            if !enemigo.activo {
                continue;
            }

            // Seleccionar sprite según el tick de explosión
            let sprite = if enemigo.explosion_tick > 0 {
                if enemigo.explosion_tick <= 2 {
                    &enemigo_1_explosion_1
                } else {
                    &enemigo_1_explosion_2
                }
            } else {
                &enemigo_1
            };

            for (dy, fila) in sprite.iter().enumerate() {
                for (dx, c) in fila.chars().enumerate() {
                    if c == '3' {
                        let ty = enemigo.y + dy;
                        let tx = enemigo.x + dx;
                        if ty < ALTO && tx < ANCHO {
                            // Si está explotando, usamos el ID 5 para pintarlo distinto
                            tablero_estados[ty][tx] =
                                if enemigo.explosion_tick > 0 { 5 } else { 3 };
                        }
                    }
                }
            }
        }

        // Estampar jugador (normal, explosión o nada)
        let jugador = ["110", "011", "110"];
        let jugador_explosion_1 = ["111", "111", "111"];
        let jugador_explosion_2 = ["000", "010", "000"];

        if estado.jugador_activo {
            let sprite = if estado.explosion_jugador_tick > 0 {
                if estado.explosion_jugador_tick <= 2 {
                    &jugador_explosion_1
                } else {
                    &jugador_explosion_2
                }
            } else {
                &jugador
            };

            let sprite_alto = sprite.len();
            let sprite_ancho = sprite[0].len();
            let origen_y = py.saturating_sub(sprite_alto / 2);
            let origen_x = px.saturating_sub(sprite_ancho / 2);

            for (dy, fila) in sprite.iter().enumerate() {
                for (dx, c) in fila.chars().enumerate() {
                    let ty = origen_y + dy;
                    let tx = origen_x + dx;
                    if ty < ALTO && tx < ANCHO && c == '1' {
                        // Si está explotando, usamos el ID 6
                        tablero_estados[ty][tx] = if estado.explosion_jugador_tick > 0 {
                            6
                        } else {
                            1
                        };
                    }
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
                // Se elimina si llega al borde derecho
                if p.x >= ANCHO - 1 {
                    p.activo = false;
                } else {
                    p.x += 1;
                }
            }
        }
        self.proyectiles.retain(|p| p.activo);
    }

    pub fn verificar_colisiones(&mut self) {
        let enemigo_1 = ["033", "330", "033"];

        for proyectil in self.proyectiles.iter_mut() {
            if !proyectil.activo {
                continue;
            }

            for enemigo in self.enemigos.iter_mut() {
                // Ignorar enemigos inactivos o que ya están explotando
                if !enemigo.activo || enemigo.explosion_tick > 0 {
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
                    proyectil.activo = false; // El proyectil desaparece
                    enemigo.explosion_tick = 1; // ¡Inicia la animación de explosión!
                    // Ya NO ponemos enemigo.activo = false aquí
                }
            }
        }

        // Limpiamos los proyectiles inactivos, pero NO los enemigos todavía
        self.proyectiles.retain(|p| p.activo);
    }
}
