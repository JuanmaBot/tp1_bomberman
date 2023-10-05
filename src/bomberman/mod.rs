pub mod punto;
use std::collections::HashSet;

use self::punto::Punto;
/// Representa un juego de Bomberman con un tablero de Strings.
pub struct Bomberman {
    pub tablero: Vec<Vec<String>>,
    pub pila_bombas: Vec<punto::Punto>,
}

impl Bomberman {
    /// Toma un Bomberman inicializado y explota la bomba ubicada en las coordenadas dadas, iniciando el juego.
    /// Devuelve un Result con un OK(()) o un Err(String) con la descripcion del mismo.
    ///
    /// # Ejemplos
    /// ```
    /// let mut bomberman = bomberman::Bomberman {tablero: tablero,pila_bombas: Vec::new()};
    /// if let Err(e) = bomberman::Bomberman::comenzar(&mut bomberman, punto_bomba.x, punto_bomba.y){
    ///     return devolver_error(e, salida)
    /// }
    /// ```
    ///
    /// # Argumentos
    ///
    /// * self: Un Bomberman con un tablero ya inicializado.
    /// * x: Coordenada X de la Bomba
    /// * y: Coordenada Y de la Bomba.
    ///
    /// # Devuelve
    ///
    /// Un Result Ok(()) si todo sale bien, o un Err(String) con la descripcion del mismo.
    pub fn comenzar(&mut self, x: usize, y: usize) -> Result<(), String> {
        let valor_casilla = &self.tablero[y][x];
        if valor_casilla.len() < 2 {
            return Err("Error: coordenadas invalidas".to_string());
        }

        let mut iter_chars = valor_casilla.chars();
        let tipo_opt = iter_chars.next();
        let alcance_str = iter_chars.next().unwrap_or('X');

        if tipo_opt != Some('B') && tipo_opt != Some('S') {
            return Err("Error: coordenadas invalidas".to_string());
        }
        let alcance = (alcance_str as usize) - ('0' as usize);
        match tipo_opt {
            Some(t) => match Self::explosion(self, x, y, alcance, t) {
                Err(e) => return Err(e),
                Ok(tab) => self.tablero = tab,
            },
            _ => return Err("Error: archivo de entrada invalido".to_string()),
        }
        if !(self.pila_bombas.is_empty()) {
            match self.pila_bombas.pop() {
                Some(p) => return Self::comenzar(self, p.x, p.y),
                _ => return Ok(()),
            }
        }
        Ok(())
    }

    /// Toma las coordenadas de una Bomba en un tablero de Bomberman con sus características y devuelve un Result con el tablero final o un Err(String) con la descripcion del mismo.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// match Self::explosion(self, x, y, alcance, tipo){
    ///     Err(e) => return Err(e),
    ///     Ok(tab) => self.tablero = tab
    /// }
    /// ```
    ///
    /// # Argumentos
    ///
    /// * self: Un Bomberman con un tablero inicializado.
    /// * x: Coordenada x de la bomba.
    /// * y: Coordenada y de la bomba.
    /// * alcance: Cantidad de celdas que puede afectar en linea recta desde las coordenadas iniciales de la bomba.
    /// * tipo: Puede ser 'B' o 'S' para indicar si es una bomba normal o de traspaso respectivamente.
    ///
    /// # Devuelve
    ///
    /// Un Result con el tablero final o en su defecto un Err(String) con la descripcion del error que ocurrio.
    fn explosion(
        &mut self,
        x: usize,
        y: usize,
        alcance: usize,
        tipo: char,
    ) -> Result<Vec<Vec<String>>, String> {
        // se llama por cada bomba que se active y devuelve el estado final del tablero

        let mut tablero_aux = self.tablero.clone();
        tablero_aux[y][x] = "_".to_string();
        let resultados: Vec<Result<(), String>> = vec![
            (Self::explosion_dirigida(
                self,
                alcance,
                Punto {
                    x: x.wrapping_sub(1),
                    y,
                },
                tipo,
                &mut tablero_aux,
                &mut HashSet::new(),
                'L',
            )),
            (Self::explosion_dirigida(
                self,
                alcance,
                Punto {
                    x,
                    y: y.wrapping_sub(1),
                },
                tipo,
                &mut tablero_aux,
                &mut HashSet::new(),
                'U',
            )),
            (Self::explosion_dirigida(
                self,
                alcance,
                Punto { x: x + 1, y },
                tipo,
                &mut tablero_aux,
                &mut HashSet::new(),
                'R',
            )),
            (Self::explosion_dirigida(
                self,
                alcance,
                Punto { x, y: y + 1 },
                tipo,
                &mut tablero_aux,
                &mut HashSet::new(),
                'D',
            )),
        ];
        for resultado in resultados {
            resultado?
        }
        Ok(tablero_aux)
    }

    /// Toma la ubicacion actual de la explosion, con algunas caracteristicas de la bomba que la creo y un set de los enemigos que ya fueron afectados por esta rama.
    /// Luego devuelve un Ok(()) o un Err(String) con la descripcion del error que lo ocasiono.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// let mut tablero_aux = self.tablero.clone();
    /// let mut resultados: Vec<Result<(),String>> = Vec::new();
    ///
    /// resultados.push(Self::explosion_dirigida(self,alcance,x.wrapping_sub(1),y,tipo,&mut tablero_aux,&mut HashSet::new(),'L',));
    /// resultados.push(Self::explosion_dirigida(self,alcance,x,y.wrapping_sub(1),tipo,&mut tablero_aux,&mut HashSet::new(),'U',));
    ///
    /// for resultado in resultados{
    ///     if let Err(e) = resultado{
    ///         return Err(e)
    ///     }
    /// }
    /// ```
    ///
    /// # Argumentos
    ///
    /// * self: Un Bomberman inicializado.
    /// * alcance: La cantidad de celdas que le quedan por recorrer a esa rafaga (0 si ya no tiene que afectar la posicion que se le pasa).
    /// * x: Coordenada x actual de la rafaga/rama de la explosion.
    /// * y: Coordenada y actual de la rafaga/rama de la explosion.
    /// * tipo: Puede ser 'B' o 'S' para indicar si es una bomba normal o de traspaso respectivamente.
    /// * tablero: Matriz de Strings mutables con los elementos de bomberman en sus casillas.
    /// * enemigos afectados: Un HashSet con las posiciones de los enemigos a los que no tiene que lastimar la rafaga/rama de la explosion.
    /// * direccion: Puede ser 'U', 'D', 'R' o 'L' para indicar si la proxima casilla esta arriba, abajo, a la derecha o a la izquierda respectivamente.
    ///
    /// # Devuelve
    ///
    /// Un Result con un Ok(()) si todo sale bien o un Err(String) con la descripcion del error que lo ocasiono.
    fn explosion_dirigida(
        &mut self,
        alcance: usize,
        punto: punto::Punto,
        tipo: char,
        tablero: &mut Vec<Vec<String>>,
        enemigos_afectados: &mut HashSet<punto::Punto>,
        direccion: char,
    ) -> Result<(), String> {
        if alcance == 0 || punto.x >= tablero.len() || punto.y >= tablero.len() {
            return Ok(());
        }

        let prox: punto::Punto = match calcular_prox(direccion, punto.x, punto.y) {
            Err(e) => return Err(e),
            Ok(pt) => pt,
        };
        let binding = tablero.clone();
        let mut iter = binding[punto.y][punto.x].as_str().chars();
        match iter.next() {
            Some('_') => {
                return self.explosion_dirigida(
                    alcance - 1,
                    Punto {
                        x: prox.x,
                        y: prox.y,
                    },
                    tipo,
                    tablero,
                    enemigos_afectados,
                    direccion,
                )
            }
            Some('D') => match iter.next() {
                Some('U') => {
                    return self.explosion_dirigida(
                        alcance - 1,
                        Punto {
                            x: punto.x,
                            y: punto.y.wrapping_sub(1),
                        },
                        tipo,
                        tablero,
                        enemigos_afectados,
                        'U',
                    )
                }
                Some('R') => {
                    return self.explosion_dirigida(
                        alcance - 1,
                        Punto {
                            x: punto.x + 1,
                            y: punto.y,
                        },
                        tipo,
                        tablero,
                        enemigos_afectados,
                        'R',
                    )
                }
                Some('L') => {
                    return self.explosion_dirigida(
                        alcance - 1,
                        Punto {
                            x: punto.x.wrapping_sub(1),
                            y: punto.y,
                        },
                        tipo,
                        tablero,
                        enemigos_afectados,
                        'L',
                    )
                }
                Some('D') => {
                    return self.explosion_dirigida(
                        alcance - 1,
                        Punto {
                            x: punto.x,
                            y: punto.y + 1,
                        },
                        tipo,
                        tablero,
                        enemigos_afectados,
                        'D',
                    )
                }
                _ => return Err("Error: archivo de entrada invalido".to_string()),
            },
            Some('R') => {
                if tipo == 'S' {
                    return self.explosion_dirigida(
                        alcance - 1,
                        Punto {
                            x: prox.x,
                            y: prox.y,
                        },
                        tipo,
                        tablero,
                        enemigos_afectados,
                        direccion,
                    );
                }
            }
            Some('B') | Some('S') => self.pila_bombas.push(punto::Punto {
                x: punto.x,
                y: punto.y,
            }),
            Some('F') => {
                match afectar_enemigo(
                    enemigos_afectados,
                    punto::Punto {
                        x: punto.x,
                        y: punto.y,
                    },
                    tablero,
                    iter.next(),
                ) {
                    Err(e) => return Err(e),
                    _ => {
                        return self.explosion_dirigida(
                            alcance - 1,
                            Punto {
                                x: prox.x,
                                y: prox.y,
                            },
                            tipo,
                            tablero,
                            enemigos_afectados,
                            direccion,
                        )
                    }
                }
            }
            Some('W') => return Ok(()),
            _ => return Err("Error: archivo de entrada invalido".to_string()),
        }
        Ok(())
    }
}

/// Toma la direccion como un char y calcula la posicion mas proxima en esa direccion a un set de coordenadas provisto.
///
/// # Ejemplos
///
/// ```
/// let prox: punto::Punto;
/// match calcular_prox(direccion, x_actual, y_actual) {
///     Err(e) => return Err(e),
///     Ok(pt) => prox = pt
/// }
/// ```
///
/// # Argumentos
///
/// * direccion: Puede ser 'U', 'D', 'R' o 'L' para indicar si la proxima casilla esta arriba, abajo, a la derecha o a la izquierda respectivamente.
/// * x: Coordenada x actual.
/// * y: Coordenada y actual.
///
/// # Devuelve
///
/// Un Result con el siguiente punto o un Err(String) con la descripcion del mismo en caso de haberle pasado mal la direccion.
fn calcular_prox(direccion: char, x: usize, y: usize) -> Result<punto::Punto, String> {
    let prox_x: usize;
    let prox_y: usize = match direccion {
        'U' => {
            prox_x = x;
            y.wrapping_sub(1)
        }
        'D' => {
            prox_x = x;
            y + 1
        }
        'R' => {
            prox_x = x + 1;
            y
        }
        'L' => {
            prox_x = x;
            y
        }
        _ => return Err("Error: archivo de entrada invalido".to_string()),
    };
    Ok(punto::Punto {
        x: prox_x,
        y: prox_y,
    })
}

/// Toma la ubicacion del enemigo a afectar con algunas de sus caracteristicas, un Set de los enemigos que ya fueron afectados y el tablero en el que esta ubicado.
///
/// # Ejemplos
///
/// ```
/// match afectar_enemigo(enemigos_afectados, punto::Punto { x: x_enemigo, y: y_enemigo }, tablero, Some(vida_enemigo_char)) {
///     Err(e) => return Err(e),
///     _ => {return self.explosion_dirigida(alcance - 1,prox.x,prox.y,tipo,tablero,enemigos_afectados,direccion)},
/// }
/// ```
///
/// # Argumentos
///
/// * enemigos_afectados: Un HashSet de los enemigos a los que no hay que afectar mas de una vez.
/// * punto: Ubicacion en el tablero del enemigo.
/// * tablero: Matriz de Strings en el que se encuentra el enemigo.
/// * opt_vida_char: Un Option con la vida restante del enemigo.
///
/// # Devuelve
///
/// Muta el tablero y devuelve un Result Ok(()) en caso de salir todo bien o un Err(String) con la descripcion del mismo en caso de que no tenga una cantidad de vida valida.
fn afectar_enemigo(
    enemigos_afectados: &mut HashSet<punto::Punto>,
    punto: punto::Punto,
    tablero: &mut [Vec<String>],
    opt_vida_char: Option<char>,
) -> Result<(), String> {
    if !enemigos_afectados.contains(&punto) {
        enemigos_afectados.insert(punto::Punto {
            x: punto.x,
            y: punto.y,
        });
        let num: char = opt_vida_char.unwrap_or('X');
        if num.is_ascii_digit() {
            let vida = (num as usize) - ('0' as usize);
            if vida == 1 {
                tablero[punto.y][punto.x] = "_".to_string()
            } else {
                tablero[punto.y][punto.x] = format!("F{}", vida - 1)
            }
        } else {
            return Err("Error: archivo de entrada invalido".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Bomberman;

    #[test]
    fn test01_bomba_explota() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "B1".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 1, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }
    #[test]
    fn test02_bombas_explotan_si_son_afectadas() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "B1".to_string(), "S1".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 1, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test03_bombas_no_afectan_si_no_alcanzan() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["B1".to_string(), "_".to_string(), "S1".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "S1".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 0, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test04_solo_bombas_traspaso_atraviesan_rocas_y_nadie_atraviesa_paredes() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["B1".to_string(), "R".to_string(), "B2".to_string()],
            vec!["_".to_string(), "_".to_string(), "R".to_string()],
            vec!["B1".to_string(), "W".to_string(), "S2".to_string()],
        ];
        let tab_final = vec![
            vec!["B1".to_string(), "R".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "R".to_string()],
            vec!["B1".to_string(), "W".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 2, 2) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(e) => {
                print!("{}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn test05_desvios_cambian_direccion() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "S1".to_string()],
            vec!["_".to_string(), "B2".to_string(), "DU".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "DU".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 1, 2) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test06_enemigos_mueren_si_no_tienen_vida() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["B1".to_string(), "F1".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 0, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test07_enemigos_no_mueren_si_tienen_vida() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["B1".to_string(), "F2".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "F1".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 0, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test08_enemigos_sufren_bombas_distintas() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["B2".to_string(), "F2".to_string(), "S1".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 0, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn test09_enemigos_no_sufren_dos_veces_la_misma_bomba() {
        let mut bomber: Bomberman;
        let tab_inicial = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["B3".to_string(), "F2".to_string(), "DL".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        let tab_final = vec![
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
            vec!["_".to_string(), "F1".to_string(), "DL".to_string()],
            vec!["_".to_string(), "_".to_string(), "_".to_string()],
        ];
        bomber = Bomberman {
            tablero: tab_inicial,
            pila_bombas: Vec::new(),
        };
        match Bomberman::comenzar(&mut bomber, 0, 1) {
            Ok(()) => assert_eq!(tab_final, bomber.tablero),
            Err(_e) => assert!(false),
        }
    }
}
