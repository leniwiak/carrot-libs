#![allow(unused_variables)]
use std::process;
use std::io;
use std::io::Write;
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{self, Event::Key, KeyEvent, KeyCode, KeyModifiers};

fn flush() {
    io::stdout().flush().expect("Failed to flush terminal output!")
}

pub fn detect() -> (&'static str, Option<char>) {
    // Go to raw mode to get more control over terminal
    crossterm::terminal::enable_raw_mode().unwrap();

    // Flush on start and end of the loop
    flush();

    // Initialize event listener
    let event = event::read().unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();

    match event {
        Key(KeyEvent {code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, ..}) => {
            ("STOP", None)
        },
        Key(KeyEvent {code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, ..}) => {
            ("STOP", None)
        },
        Key(KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::NONE, ..}) => {
            ("LEFT", None)
        },
        Key(KeyEvent {code: KeyCode::Right, modifiers: KeyModifiers::NONE, ..}) => {
            ("RIGHT", None)
        },
        Key(KeyEvent {code: KeyCode::Up, modifiers: KeyModifiers::NONE, ..}) => {
            ("UP", None)
        },
        Key(KeyEvent {code: KeyCode::Down, modifiers: KeyModifiers::NONE, ..}) => {
            ("DOWN", None)
        },
        Key(KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, ..}) => {
            ("CTRL-LEFT", None)
        },
        Key(KeyEvent {code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, ..}) => {
            ("CTRL-RIGHT", None)
        },
        Key(KeyEvent {code: KeyCode::Home, ..}) => {
            ("HOME", None)
        },
        Key(KeyEvent {code: KeyCode::End, ..}) => {
            ("END", None)
        },
        Key(KeyEvent {code: KeyCode::Backspace, modifiers: KeyModifiers::NONE, ..}) => {
            ("BACKSPACE", None)
        },
        Key(KeyEvent {code: KeyCode::Char(h), modifiers: KeyModifiers::CONTROL, ..}) => {
            ("CTRL-BACKSPACE", None)
        },
        Key(KeyEvent {code: KeyCode::Delete, modifiers: KeyModifiers::NONE, ..}) => {
            ("DEL", None)
        },
        Key(KeyEvent {code: KeyCode::Delete, modifiers: KeyModifiers::CONTROL, ..}) => {
            ("CTRL+DEL", None)
        },
        Key(KeyEvent {code: KeyCode::Enter, ..}) => {
            ("ENTER", None)
        },
        Key(KeyEvent {code: KeyCode::Esc, ..}) => {
            ("ESCAPE", None)
        },
        Key(KeyEvent {code: KeyCode::Char(c), ..}) => {
            ("CHAR", Some(c))
        },
        _ => {
            ("UNKNOWN", None)
        }
    }
}

pub fn ask<S: AsRef<str>>(text: S) -> Result<bool, String> {
    match get(format!("{} [y/n]: ", text.as_ref()), false) {
        Err(e) => {
            Err(format!("Can't get user input: {}", e))
        }
        Ok(ret) => {
            let toclear:bool;
            let input = ret;
            if input.len() != 1 {
                println!("Sorry! I don't undestand your input.");
                return ask(opt);
            }
            let lowercased_input = input[0].trim().to_lowercase();
            if lowercased_input == "y" || lowercased_input == "yes" { toclear = true; }
            else if lowercased_input == "n" || lowercased_input == "no" { toclear = false; }
            else { println!("I don't undestand! Try again."); return ask(opt); }
            Ok(toclear)
        }
    }
    
}

pub fn get<S: AsRef<str>>(prompt:S,secure:bool) -> Result<Vec<String>, String> {
    get_with_default(prompt.as_ref().to_owned(), secure, None, None)
}

pub fn get_with_default(prompt:String,secure:bool,starting_value:Option<String>,starting_curpos:Option<usize>) -> Result<Vec<String>, String> {
    // FOR ALL COMMENTS BELLOW: Assume, that user typed this command into a shell: af file then ad dir
    // This variable contains full line typed by the user (List 1.: 'af file then ad dir')
    let mut input:Vec<char> = Vec::new();

    // Print a prompt
    print!("{prompt}");
    if secure {
        print!("This prompt is secured. You're typing silently.");
    }
    // Print starting value if any
    if let Some(ref v) = starting_value {
        print!("{v}")
    }
    
    // Flush stdout to print the prompt
    if io::stdout().flush().is_err() {
        Err("Cannot flush terminal".to_string())
    } else {
        // Read line into "input"
        // Process each character written on keyboard

        // Get the cursor position when we've started
        let initial_cur_pos = crossterm::cursor::position().expect("Failed to obtain cursor position!").0;
        // This is going to indicate where to add new letters to "input"
        // If starting_value is defined - put the cursor in position defined by starting_curpos or at the
        // end of starting_value if starting_curpos is not defined.
        let mut idx = if let Some(ref v) = starting_value {
            starting_curpos.unwrap_or(v.len())
        }
        // If starting_value isn't set, just put our cursor at start of the prompt
        else {
            0
        };

        loop {
            // Show prompt and contents of input
            if !secure {
                // Move to start of the column
                print!("\r");
                // Clear everything on that line
                print!("{}", Clear(ClearType::CurrentLine));
                let input_string = input.iter().collect::<String>();
                print!("{}{}", prompt, input_string);
                // Move cursor to position defined in "idx" + "initial_cur_pos"
                // Flush on start and end of the loop
                print!("{}", crossterm::cursor::MoveToColumn(idx as u16 +initial_cur_pos)); 
            }
            flush();

            let (key_type, letter) = detect();
            // Check event
            match key_type {
                // CTRL+Z or CTRL+D: Quit
                "STOP" => {
                    // Inspired by BASH
                    // Don't exit unless the prompt is empty
                    if input.is_empty() {
                        // Disable raw mode and quit
                        crossterm::terminal::disable_raw_mode().expect("Cannot quit from raw terminal mode!");
                        println!();
                        process::exit(1);
                    }
                },

                // ARROWS without CTRL: Cursor movement
                "LEFT" => {
                    if idx != 0 {
                        // Move cursor to left
                        idx -= 1;
                    } else {print!("\x07");continue;};
                    
                },
                "RIGHT" => {
                    if idx != input.len() {
                        // Move cursor to right
                        idx += 1;
                    } else {print!("\x07");continue;};
                },

                // CTRL+ARROW: Move cursor to the next whitespace
                "CTRL+LEFT" => {
                    while idx != 0 {
                        idx -= 1;
                        if input[idx].is_whitespace() { break }
                    }
                }
                "CTRL+RIGHT" => {
                    while idx != input.len() {
                        idx += 1;
                        if idx == input.len() || input[idx].is_whitespace() { break }
                    }
                }
                
                // HOME and END keys support
                "HOME" => {
                    // Move cursor back to the prompt
                    idx=0;
                }
                "END" => {
                    // Move where "input" is reaching it's end
                    idx=input.len();
                }

                // BACKSPACE: Remove character before cursor
                "BACKSPACE" => {
                    if idx != 0 {
                        if idx != input.len() {
                            input.remove(idx-1);
                        }
                        else {
                            // Since removing from the last index is impossibl, use "pop" when user wants
                            // to remove the last character from the input
                            input.pop();
                        };
                        // Move cursor
                        idx -= 1;
                    } else {print!("\x07")};
                },
                // CTRL+BACKSPACE: Remove character before cursor until whitespace
                // FUNFACT: Terminal emulators on Linux detect CTRL+Backspace as CTRL+H
                // The code below is correct. Don't change KeyCode::Char to KeyCode::Spacebar
                "CTRL+BACKSPACE" => {
                    while idx > 0 {
                        if !input[idx-1].is_whitespace() {
                            input.remove(idx-1);
                        }
                        else {
                            // Remove the remaining white space
                            input.remove(idx-1);
                            idx-=1;
                            break;
                        }
                        idx-=1;
                    }
                },

                // DEL: Remove character on cursor
                "DEL" => {
                    if idx != input.len() {
                        input.remove(idx);
                    } else {print!("\x07")};
                },
                // CTRL+DEL: Remove all characters after cursor until whitespace
                "CTRL+DEL" => {
                    while idx < input.len() {
                        if !input[idx].is_whitespace() {
                            input.remove(idx);
                        }
                    }
                },

                // ENTER: Quickly append newline character to "input" and stop waiting for input by breaking out of the loop
                "ENTER" => {
                    input.push('\n');
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                },
               
                // OTHER
                "UNKNOWN" => {
                    // Bell!
                    print!("\x07");
                },

                // ANY CHARACTER WITHOUT CTRL: Show it on keyboard and add it to "input" variable
                "CHAR" => {
                    // Insert a char in "input" on position where the cursor is located + the number 
                    input.insert(idx, letter.expect(""));
                    // Move cursor to the right as we type
                    idx +=1;
                },
                _ => {
                    eprintln!("Impossible :o");
                    process::exit(1);
                }
            };
        };
        // Quit from raw mode when we're out of the loop
        print!("\n\r");
        let input_string = input.iter().collect::<String>();
        Ok(input_string.split_whitespace().map(str::to_string).collect())
    }
}
