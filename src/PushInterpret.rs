/*
 * Interprete para Push
 * Suitscape© Latam 2026 (SpacesXD)
 */

// espacio reservado de modulo publico
pub mod PushInterpret {

    // funcion principal para convertir el codigo a oush bytecode
    pub fn ConvertToBytecode(code: &str) {
        let mut blog: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        let mut is_int_decl: bool = false;
        let mut int_stage: i32 = 0;
        let mut current_var_name: String = String::from("");

        println!("\x1b[00mConverting to PushBytecode...");
        
        for token in code.split_whitespace() {
            if token == "io->print(" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "io->endl" {
                blog.push_str("endl \n");
            } else if token == "io->white" {
                blog.push_str("col white ");
            } else if token == "io->black" {
                blog.push_str("col black ");
            } else if token == "io->red" {
                blog.push_str("col red ");
            } else if token == "io->green" {
                blog.push_str("col green ");
            } else if token == "io->blue" {
                blog.push_str("col blue ");
            } else if token == "io->yellow" {
                blog.push_str("col yellow ");
            } else if token == "int" {
                is_int_decl = true;
                int_stage = 1; // Espera el nombre de la variable
            } else if is_int_decl {
                if int_stage == 1 {
                    current_var_name = token.to_owned();
                    int_stage = 2; // Espera el '='
                } else if int_stage == 2 && token == "=" {
                    int_stage = 3; // Espera el valor
                } else if int_stage == 3 {
                    blog.push_str(&format!("int {} {} \n", current_var_name, token));
                    is_int_decl = false;
                    int_stage = 0;
                }
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        if token == ")" || token == "/*" || token == "*/" {
                            continue;
                        } else {
                            blog.push_str(&("lg ".to_owned() + &token.to_owned() + " \n"));
                        }  
                    }
                }
            }
        }

        let namefile: String = crate::PkgManager::PkgManager::nmainfile() + ".pb";
        std::fs::write(namefile, blog).expect("Unable to write file");
    }

    pub fn ConvertToC() {
        let namefile_pb = crate::PkgManager::PkgManager::nmainfile() + ".pb";
        let code = std::fs::read_to_string(&namefile_pb).unwrap_or_default();
        let mut blog: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        let mut is_int: bool = false;
        let mut int_var_name: String = String::from("");

        blog.push_str("#include <stdio.h>\n// Template made by SpacesXD \n\nint main(void) {\n");
        
        for token in code.split_whitespace() {
            if token == "lg" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "endl" {
                blog.push_str("    printf(\"\\n\");\n");
            } else if token == "col" {
                cmdf = false;
                cmdt = String::from("color");
            } else if token == "int" {
                cmdf = false;
                cmdt = String::from("variable");
                is_int = true;
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        blog.push_str(&("    printf(\"".to_owned() + &token.to_owned() + " \");\n"));
                    }
                } else if cmdt == "color" {
                    if !cmdf {
                        if token == "white" {
                            blog.push_str("    printf(\"\\033[00m\");\n");
                        } else if token == "black" {
                            blog.push_str("    printf(\"\\033[30m\");\n");
                        } else if token == "red" {
                            blog.push_str("    printf(\"\\033[31m\");\n");
                        } else if token == "green" {
                            blog.push_str("    printf(\"\\033[32m\");\n");
                        } else if token == "blue" {
                            blog.push_str("    printf(\"\\033[34m\");\n");
                        } else if token == "yellow" {
                            blog.push_str("    printf(\"\\033[33m\");\n");
                        }
                    }
                } else if cmdt == "variable" {
                    if is_int {
                        int_var_name = token.to_owned();
                        is_int = false;
                    } else {
                        blog.push_str(&format!("    int {} = {};\n", int_var_name, token));
                        cmdf = true;
                    }
                }
            }
        }

        blog.push_str("\n    return 0;\n}");
        let namefile: String = crate::PkgManager::PkgManager::nmainfile() + ".c";
        std::fs::write(&namefile, blog).expect("Unable to write file");
    }
    
    pub fn InterpretBytecode() -> String {
        let namefile_pb = crate::PkgManager::PkgManager::nmainfile() + ".pb";
        let code: String = std::fs::read_to_string(&namefile_pb).unwrap_or_default();
        let mut log: String = String::from("");
        
        let mut vars_name: Vec<String> = Vec::new();
        let mut vars_value_int: Vec<i32> = Vec::new();

        let mut cmdt: String = String::from("");
        let mut vrbt: String = String::from("");
        let mut wv: bool = false;
        let mut cmdf: bool = false;
        
        println!("\x1b[00mInterpreting PushBytecode...");
        
        for token in code.split_whitespace() {
            if token == "lg" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "endl" {
                log.push_str("\n");
            } else if token == "col" {
                cmdf = false;
                cmdt = String::from("color");
            } else if token == "int" {
                cmdf = false;
                cmdt = String::from("variable");
                vrbt = String::from("int");
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        let mut found = false;
                        for i in 0..vars_name.len() {
                            if vars_name[i] == token {
                                log.push_str(&(vars_value_int[i].to_string() + " "));
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            log.push_str(&(token.to_owned() + " "));
                        }
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
                            log.push_str("\x1b[32m");
                        } else if token == "blue" {
                            log.push_str("\x1b[34m");
                        } else if token == "yellow" {
                            log.push_str("\x1b[33m");
                        }
                        cmdf = true;
                    }
                } else if cmdt == "variable" {
                    if !cmdf {
                        if vrbt == "int" && !wv {
                            vars_name.push(token.to_owned());
                            wv = true;
                        } else if wv {
                            if let Ok(val) = token.parse::<i32>() {
                                vars_value_int.push(val);
                            } else {
                                vars_value_int.push(0);
                            }
                            wv = false;
                            cmdf = true;
                        }
                    }
                }
            }
        }
        return log;
    }

    // funcion principal para interpretar el codigo
    pub fn interpret(code: &str) -> String {
        let mut log: String = String::from("");
        
        let mut vars_name: Vec<String> = Vec::new();
        let mut vars_value_int: Vec<i32> = Vec::new();
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        let mut currints: i32 = 0;
        let mut err: i32 = 0;
        
        let mut is_int_decl: bool = false;
        let mut int_stage: i32 = 0;
        let mut current_var_name: String = String::from("");
        
        println!("\x1b[00mInterpreting...");
        
        for token in code.split_whitespace() {
            if token == "io->print(" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "io->endl" {
                log.push_str("\n");
            } else if token == "/*" {
                cmdf = false;
                cmdt = String::from("comment");
            } else if token == "*/" {
                if !cmdf {
                    cmdf = true;
                    cmdt = String::from("");
                }
            } else if token == "io->white" {
                log.push_str("\x1b[00m");
            } else if token == "io->black" {
                log.push_str("\x1b[30m");
            } else if token == "io->red" {
                log.push_str("\x1b[31m");
            } else if token == "io->green" {
                log.push_str("\x1b[32m");
            } else if token == "io->blue" {
                log.push_str("\x1b[34m");
            } else if token == "io->yellow" {
                log.push_str("\x1b[33m");
            } else if token == ")" {
                if !cmdf {
                    cmdf = true;
                    cmdt = String::from("");
                }
            } else if token == "int" {
                is_int_decl = true;
                int_stage = 1;
            } else if is_int_decl {
                if int_stage == 1 {
                    current_var_name = token.to_owned();
                    int_stage = 2;
                } else if int_stage == 2 && token == "=" {
                    int_stage = 3;
                } else if int_stage == 3 {
                    if let Ok(val) = token.parse::<i32>() {
                        vars_name.push(current_var_name.clone());
                        vars_value_int.push(val);
                    } else {
                        err += 1;
                        log.push_str(&format!("\x1b[31m\n![CODE ERROR] Expected integer value, got '{}'\n", token));
                    }
                    is_int_decl = false;
                    int_stage = 0;
                }
            } else {
                if cmdt == "print" {
                    if !cmdf {
                        let mut found = false;
                        for i in 0..vars_name.len() {
                            if vars_name[i] == token {
                                log.push_str(&(vars_value_int[i].to_string() + " "));
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            log.push_str(&(token.to_owned() + " "));
                        }
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
             ConvertToBytecode(code);
             ConvertToC();
        } else { 
            log.push_str("[CODE ERROR] Due to code errors, it won't be interpretable :("); 
        }

        return log;
    }
}