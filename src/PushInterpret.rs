/*
 * Interprete para Push
 * Suitscape© Latam 2026 (SpacesXD)
 */

// espacio reservado de modulo publico
pub mod PushInterpret {
    
    // funcion principal para convertir el codigo a oush bytecode
    // a quien le miento es un copy paste de la funcion de interpretacion principal xd
    pub fn ConvertToBytecode(code: &str) {
        let mut blog: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        println!("\x1b[00mConverting to PushBytecode...");
        
        for token in code.split_whitespace() {
            if token == "io->print(" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "io->endl" {
                blog.push_str("endl ");
            } else if token == "io->white" {
                blog.push_str("col white ");
            } else if token == "io->black" {
                blog.push_str("col black ");
            } else if token == "io->red" {
                blog.push_str("col red ");
            } else if token == "io->green" {
                blog.push_str("col green ")
            } else if token == "io->blue" {
                blog.push_str("col blue ")
            } else if token == "io->yellow" {
                blog.push_str("col yellow ")
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        if token == ")" || token == "/*" || token == "*/" {
                            continue;
                        } else {
                            blog.push_str(&(" lg ".to_owned() + &token.to_owned()));
                        }  
                    }
                }
            }
        }

        let namefile: String = crate::PkgManager::PkgManager::nmainfile() + ".pb";
        std::fs::write(&namefile, blog).expect("Unable to write file");
    }

    pub fn ConvertToC(code: &str) {
        let mut blog: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        blog.push_str("#include <stdio.h>\n// Template made by SpacesXD \n\nint main(void) {\n");
        
        for token in code.split_whitespace() {
            if token == "lg" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "endl" {
                blog.push_str("\n");
            } else if token == "col" {
                cmdf = false;
                cmdt = String::from("color");
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        blog.push_str(&("    printf(\"".to_owned() + &token.to_owned() + "\");\n"));
                    }
                } else if cmdt == "color" {
                    if !cmdf {
                        if token == "white" {
                            blog.push_str("\033[00m");
                        } else if token == "black" {
                            blog.push_str("\033[30m");
                        } else if token == "red" {
                            blog.push_str("\033[31m");
                        } else if token == "green" {
                            blog.push_str("\033[32m");
                        } else if token == "blue" {
                            blog.push_str("\033[34m");
                        } else if token == "yellow" {
                            blog.push_str("\033[33m");
                        }
                    }
                }
            }
        }

        blog.push_str("\n   return 0;\n}");
        let namefile: String = crate::PkgManager::PkgManager::nmainfile() + ".c";
        std::fs::write(&namefile, blog).expect("Unable to write file");
    }
    
    pub fn InterpretBytecode() -> String {
        let code: String = std::fs::read_to_string("main.pb").expect("Unable to read file");
        let mut log: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        println!("\x1b[00mInterpreting PushBytecode...");
        
        for token   in code.split_whitespace() {
            if token == "lg" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "endl" {
                log.push_str("\n");
            } else if token == "col" {
                cmdf = false;
                cmdt = String::from("color");
            } else {
                if cmdt == "print" {
                    if !cmdf {
                      log.push_str(&(token.to_owned() + " "));
                    }
                } else if cmdt == "color" {
                    if !cmdf {
                        if token == "white" {
                            log.push_str("\x1b[00m");
                        } else if token == "black" {
                            log.push_str("\x1b[30m");
                        } else if token == "red" {
                            log.push_str("\x1b[31m");
                        } else if token == "green" {
                            log.push_str("\x1b[32m")
                        } else if token == "blue" {
                            log.push_str("\x1b[34m")
                        } else if token == "yellow" {
                            log.push_str("\x1b[33m")
                        }
                            cmdf = true;
                    }
                }
            }
        }
        return log
    }

    // funcion principal para interpretar el codigo
    pub fn interpret(code: &str) -> String {
        // el registro de outputs
        let mut log: String = String::from("");
        
        // variables para el interprete
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        let mut currints: i32 = 0;
        let mut err: i32 = 0;
        
        // cabecera
        println!("\x1b[00mInterpreting...");
        
        // por cada token del codigo...
        for token in code.split_whitespace() {
            // si el token es un print...
            if token == "io->print(" {
                // instruccion terminada? no, tipo de instruccion? print
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "io->endl" {
                // para el salto de linea
                log.push_str("\n");
            } else if token == "/*" {
                // inicio de comentario
                // instruccion terminada? no, tipo de instruccion? comentario
                cmdf = false;
                cmdt = String::from("comment");
            } else if token == "*/" {
                // terminado de comentario
                // si... comentario terminado
                if !cmdf {
                    // el comentario termino aca y no hay instruccion
                    cmdf = true;
                    cmdt = String::from("");
                }
            } else if token == "io->white" {
                // todo esto y los demas solo es para colores en consola
                log.push_str("\x1b[00m");
            } else if token == "io->black" {
                log.push_str("\x1b[30m");
            } else if token == "io->red" {
                log.push_str("\x1b[31m");
            } else if token == "io->green" {
                log.push_str("\x1b[32m")
            } else if token == "io->blue" {
                log.push_str("\x1b[34m")
            } else if token == "io->yellow" {
                log.push_str("\x1b[33m")
            } else if token == ")"{
                // esto es para terminar el print u otra operacion que necesite ()
                // esto es copy paste del */ jeje
                if !cmdf {
                    cmdf = true;
                    cmdt = String::from("");
                }
            } else {
                /* esto pregunta basicamente:
                 * interprete: ve "Hello"
                 * interprete: ve que no es ninguna instruccion
                 * interprete: antes de madar error checa si ya hay una instruccion activa
                 * si esta activa es argumento de lo contrario es error
                 */
                if cmdt == "print" {
                    if !cmdf {
                      log.push_str(&(token.to_owned() + " "));
                    }
                } else if cmdt == "comment" {
                    if !cmdf {
                      log.push_str("");  
                    }
                } else {
                    err += 1;
                    log.push_str(&format!("\x1b[31m\n![CODE ERROR] Instruction {} was not recognized\n", currints));
                }
            }
            currints += 1;
        }

        if err < 1 {
             let cc = std::fs::read_to_string(crate::PkgManager::PkgManager::nmainfile() + ".pb").expect("Unable to read file");
             ConvertToC(&cc);
        } else { 
            log.push_str("[CODE ERROR] Due to code errors, it won't be interpretable :("); 
        }

        return log;
    }
}