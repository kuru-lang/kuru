use std::io::{self, stdout, Write, Read};
use std::process::{Command, Child, Stdio};
use std::path::Path;
use std::env;

use whoami;
use termios::{self, Termios, TCSANOW, tcsetattr};

mod color;
mod parse;

fn print_header() {
    color::set(true, color::Red, color::Black);
    print!("jeronaldaron.");
    color::set(true, color::Blue, color::Black);
    print!("{} ", env!("CARGO_PKG_NAME"));
    color::set(false, color::Green, color::Black);
    println!(env!("CARGO_PKG_VERSION"));
    color::set(true, color::White, color::Black);
    print!("Copyright © ");
    color::set(false, color::Magenta, color::Black);
    println!("Jeron Lau 2018 - 2019.");
    color::set(true, color::White, color::Black);
    print!("License ");
    color::set(false, color::Magenta, color::Black);
    println!("MIT / BSL-1.0");
    color::set(false, color::White, color::Black);
    println!();
}


/*
struct Context {
    window: Box<>,
}



fn main() -> Result<(), std::io::Error> {
    // Init pancurses
    let window = initscr();
    window.nodelay(true);
    noecho();
    if has_colors() {
        start_color();
    }
    for (i, color) in COLOR_TABLE.into_iter().enumerate() {
        init_pair(i as i16, *color, COLOR_BLACK);
    }
    let lines = window.get_max_y();
    let cols = window.get_max_x();

    // 
    let mut context = Context {
        window, lines, cols,
    };

    print_header(&mut context);

    let stdin = io::stdin();
    let mut input = stdin.lock();

    context.err.fg(term::color::RED).unwrap();
    context.out.fg(term::color::WHITE).unwrap();

    // Main loop.
    'main: loop {
        window.mv(context.lines - 1, 0);
        window.addstr(" >: ");

        let mut buffer = String::new();
        input_loop(&mut buffer);

        // Built-in Functions:
        match buffer.as_str() {
            "quit" => break 'main,
            _ => {
                window.mv(context.lines - 2, 0);
                window.addstr("Unknown function!")
            },
        }
    }

    // Quit
    endwin();
    Ok(())
}*/

fn help() {
    println!("Nahar interactive shell.

      cd .directory/               # Change directory.
      \"NOTE text\"                  # Print out string (stdout).  Special codes for graphics.
      warn \"WARN text\"             # Print out warning (stderr).
      fail \"FAIL text\"             # Print out error & exit (stderr).
      quit return_var              # Exit on success.
    ");
}

fn folder_from_path(path: &String) -> String {
    Path::new(&path).file_name().and_then(|f| {Some(f.to_str().unwrap_or(&path).to_string())}).unwrap_or(path.clone())
}

fn main() -> Result<(), std::io::Error> {
    let termios = Termios::from_fd(0).unwrap();
    let mut new_termios = termios.clone();
    termios::cfmakeraw(&mut new_termios);
    let mut reader = io::stdin();
    let mut input: String = "".to_string();
    let mut input_vec = Vec::new();
    let mut alt = false;
    let mut arrow = false;
    let mut cursor = 0;
    let username = whoami::username();
    let user = whoami::user().split_whitespace().next().unwrap_or(&username).to_string();
    let hostname = whoami::hostname();
    let mut path = std::env::var("HOME").unwrap();
    let mut folder = folder_from_path(&path);

    env::set_current_dir(&path).unwrap();

    print_header();
    print!("\x1B]0;Nahar Shell: {}@{}:{}\x07", username, hostname, path);
    stdout().flush()?;

    'main: loop {
        tcsetattr(0, TCSANOW, &mut new_termios).unwrap();

        // Update string.
        input = input_vec.iter().collect();

        'read: loop {
            print!("\x1B[2K\r"); // Reset line
            print!("{}({})$ {}", user, folder, input);
            stdout().flush()?;
            let mut buffer = [0; 1];
            reader.read_exact(&mut buffer).unwrap();
            alt = match buffer[0] {
                b'\x0d' => break 'read,
                // Control Operations (a-z)
                b'\x01' => {
                    false
                }, // Ctrl-A prompt select all
                b'\x03' => {
                    false
                }, // Ctrl-C prompt copy
                b'\x09' => {
                    false
                }, // Tab autocomplete
                b'\x13' => {
                    false
                }, // Ctrl-S prompt share session history
                b'\x16' => {
                    false
                }, // Ctrl-V prompt paste
                b'\x17' => {
                    false
                }, // Ctrl-W prompt quit y/n
                b'\x18' => {
                    false
                }, // Ctrl-X prompt cut
                b'\x1a' => {
                    false
                }, // Ctrl-Z prompt undo
                // 
                b'\x1b' => true, // Alt
                b'\x5b' => {
                    if alt {
                        arrow = true;
                    }
                    false
                }, // 
                b'\x7e' => { // delete
                    input_vec.pop();
                    false
                },
                b'\x7f' => {
                    input_vec.pop();
                    false
                }, // backspace
//                
                character => {
                    if arrow {
                        match character {
                            b'\x41' => { println!("TODO: Up") }, // Up
                            b'\x42' => { println!("TODO: Down") }, // Down
                            b'\x43' => { println!("TODO: Right") }, // Right
                            b'\x44' => { println!("TODO: Left") }, // Left
                            _ => { println!("Unknown key sequence") }, // Unknown
                        }
                        arrow = false;
                    } else {
                        input_vec.push(character as char);
                    }
                    false
                },
            };

            // Update string.
            input = input_vec.iter().collect();
        };

        input_vec.clear();
        tcsetattr(0, TCSANOW, &termios).unwrap();
        println!("");

        // must be peekable so we know when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {
            let mut parts = parse::parse(command);
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}: {:?}", e, root);
                    }
                    path = root.to_str().and_then(|f| {Some(f.to_string())}).unwrap();
                    folder = folder_from_path(&path);
                    previous_command = None;
                },
                "warn" => {
                    eprintln!("");
                }
                "fail" => {
                    eprintln!("");
                }
                "quit" => break 'main,
                "help" => help(),
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    // Handle piping.
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait()?;
        }
    }

    // Reset attributes and colors
    tcsetattr(0, TCSANOW, &termios).unwrap();
    color::reset();

    // Quit
    Ok(())
}
