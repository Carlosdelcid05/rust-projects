mod juegos;

use juegos::menu;

fn main() {
    // Bucle del menú principal: tras terminar una partida volvemos a mostrar
    // el menú, salvo que el usuario haya elegido "Salir".
    loop {
        let opcion = menu::mostrar_menu();
        let continuar = menu::lanzador(opcion);
        if !continuar {
            break;
        }
    }
}
