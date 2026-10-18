/*
 * Administrador de paquetes de Push
 * Suitscape© Latam 2026
 */

// Espacio Reservado de Modulo
pub mod PkgManager {
    
    // Funcion para comprobar que el administrador de paquetes este bien
    pub fn check_pkgmanager() {
        //variables para leer el archico de pkgmanager
        let input = std::fs::read_to_string("pkgmanager.txt").expect("xd");
        let mut vf = false;
        
        //log de test
        println!("Loading PkgManager...");
        
        //parser/mini interpretador
        for token in input.split_whitespace() {
            //solo comprueba la verison
            if token == "version:" {
                vf = true;
            } else if vf {
                vf = false;
                if token == 5.to_string() {
                    println!("\x1b[32mSuccesly Checked!");
                } else {
                    println!("\x1b[31mInvalid PkgManager Version");
                }
            }
        }
    }
    
    //devuelve el codigo del archivo principal
    pub fn mainfile() -> String {
        let input = std::fs::read_to_string("pkgmanager.txt").expect("xd");
        let mut mf = false;
        
        for token in input.split_whitespace() {
            if token == "mainfile:" {
                mf = true;
            } else if mf {
                mf = false;
                return std::fs::read_to_string(token).expect("Nose:P esto era necessrio xd");
            }
        }
        
        String::new()
    }
    
    pub fn nmainfile() -> String {
        let input = std::fs::read_to_string("pkgmanager.txt").expect("xd");
        let mut mf = false;
        
        for token in input.split_whitespace() {
            if token == "mainfile:" {
                mf = true;
            } else if mf {
                mf = false;
                return token.to_string();
            }
        }
        
        String::new()
    }

    //devuelve el nombre del autor del codigo
    pub fn author() -> String {
        let input = std::fs::read_to_string("pkgmanager.txt").expect("xd");
        let mut at = false;
        
        for token in input.split_whitespace() {
            if token == "author:" {
                at = true;
            } else if at {
                at = false;
                return token.to_string();
            }
        }
        
        String::new()
    }
}