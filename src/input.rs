#![allow(unused_variables)]
use std::process;
use std::io;
use std::io::Write;
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{self, Event::Key, KeyEvent, KeyCode, KeyModifiers};

fn flush() {
    io::stdout().flush().expect("Failed to flush terminal output!")
}

pub fn get(prompt:String) -> Vec<String> {
    // FOR ALL COMMENTS BELLOW: Assume, that user typed this command into a shell: af file then ad dir
    // This variable contains full line typed by the user (List 1.: 'af file then ad dir')
    let mut input:Vec<char> = Vec::new();

    // This list contains arguments passed by the user and with all built-in commands separated 
    // (List 1.: 'af', 'file') (List 2.: 'then') (List 3.: 'ad', 'dir')
    let words: Vec<String> = Vec::new();
    
    // Print a prompt
    print!("{prompt}");

    // Flush stdout to print the prompt
    io::stdout().flush().expect("Cannot flush output!");
        // Read line into "input"
        // Process each character written on keyboard

        // Get the cursor position when we've started
        let initial_cur_pos = crossterm::cursor::position().expect("Failed to obtain cursor position!").0;
        // This is going to indicate where to add new letters to "input"
        let mut idx = 0;

        loop {
            // Go to raw mode to get more control over terminal
            crossterm::terminal::enable_raw_mode().unwrap();

            // Flush on start and end of the loop
            flush();

            // Initialize event listener
            let event = event::read().unwrap();
            // Check event
            match event {
                // CTRL+Z: Quit
                Key(KeyEvent {code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, ..}) => {
                    // Inspired by BASH
                    // Don't exit unless the prompt is empty
                    if input.is_empty() {
                        // Disable raw mode and quit
                        crossterm::terminal::disable_raw_mode().expect("Cannot quit from raw terminal mode!");
                        println!();
                        process::exit(1);
                    }
                },

                // CTRL+D: Quit
                Key(KeyEvent {code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, ..}) => {
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
                Key(KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::NONE, ..}) => {
                    if idx != 0 {
                        // Move cursor to left
                        idx -= 1;
                    } else {print!("\x07");continue;};
                    
                },
                Key(KeyEvent {code: KeyCode::Right, modifiers: KeyModifiers::NONE, ..}) => {
                    if idx != input.len() {
                        // Move cursor to right
                        idx += 1;
                    } else {print!("\x07");continue;};
                },

                // CTRL+ARROW: Move cursor to the next whitespace
                Key(KeyEvent {code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, ..}) => {
                    while idx != 0 {
                        idx -= 1;
                        if input[idx].is_whitespace() {
                            break
                        }
                    }
                }
                Key(KeyEvent {code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, ..}) => {
                    while idx != input.len() {
                        idx += 1;
                        if idx == input.len() || input[idx].is_whitespace() { break }
                    }
                }
                
                // HOME and END keys support
                Key(KeyEvent {code: KeyCode::Home, ..}) => {
                    // Move cursor back to the prompt
                    idx=0;
                }
                Key(KeyEvent {code: KeyCode::End, ..}) => {
                    // Move where "input" is reaching it's end
                    idx=input.len();
                }

                // BACKSPACE: Remove character before cursor
                Key(KeyEvent {code: KeyCode::Backspace, modifiers: KeyModifiers::NONE, ..}) => {
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
                Key(KeyEvent {code: KeyCode::Char(h), modifiers: KeyModifiers::CONTROL, ..}) => {
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
                Key(KeyEvent {code: KeyCode::Delete, modifiers: KeyModifiers::NONE, ..}) => {
                    if idx != input.len() {
                        input.remove(idx);
                    } else {print!("\x07")};
                },
                // CTRL+DEL: Remove all characters after cursor until whitespace
                Key(KeyEvent {code: KeyCode::Delete, modifiers: KeyModifiers::CONTROL, ..}) => {
                    while idx < input.len() {
                        if !input[idx].is_whitespace() {
                            input.remove(idx);
                        }
                    }
                },

                // ENTER: Quickly append newline character to "input" and stop waiting for input by breaking out of the loop
                Key(KeyEvent {code: KeyCode::Enter, ..}) => {
                    input.push('\n');
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                },

                // ANY CHARACTER WITHOUT CTRL: Show it on keyboard and add it to "input" variable
                Key(KeyEvent {code: KeyCode::Char(c), ..}) => {
                    // Insert a char in "input" on position where the cursor is located + the number 
                    input.insert(idx, c);
                    // Move cursor to the right as we type
                    idx +=1;
                },
               
                // OTHER
                _ => {
                    // Bell!
                    print!("\x07");
                },
            };
            // Move to start of the column
            print!("\r");
            // Clear everything on that line
            print!("{}", Clear(ClearType::CurrentLine));
            // Show prompt and contents of input
            let input_string = input.iter().collect::<String>();
            print!("{}{}", prompt, input_string);
            // Move cursor to position defined in "idx" + "initial_cur_pos"
            print!("{}", crossterm::cursor::MoveToColumn(idx as u16 +initial_cur_pos)); 
            // Flush on start and end of the loop
            flush();
        };
        // Quit from raw mode when we're out of the loop
        print!("\n\r");
        crossterm::terminal::disable_raw_mode().unwrap();
        /*
            Character division helps to find individual arguments (words)
            Expected output: ('af' 'file' 'then' 'ad' 'dir')
        */
        let input_string = input.iter().collect::<String>();
        input_string.split_whitespace().map(str::to_string).collect()
}
