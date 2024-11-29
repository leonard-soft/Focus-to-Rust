use crossterm::terminal::{self, enable_raw_mode, disable_raw_mode};
use rodio::{Decoder, OutputStream, source::Source};
use std::io::{self, Write};
use std::process::Command;
use std::{thread};
use std::fs::File;
use std::time::{Duration, Instant};
use crossterm::{
    cursor,
    event::{self},
    execute,
    terminal::{ClearType},
};

fn play_sound() {
    let archivo = File::open("/usr/share/focus/static/sound.wav").expect("No se pudo abrir el archivo de audio");
    let (_stream, stream_handle) = OutputStream::try_default().expect("No se pudo obtener el flujo de salida");
    let source = Decoder::new_wav(archivo).expect("No se pudo decodificar el archivo de audio");
    stream_handle.play_raw(source.convert_samples()).expect("No se pudo reproducir el audio");
    std::thread::sleep(std::time::Duration::from_secs(3));
}


fn wait_key() {
    enable_raw_mode().unwrap();
    loop {
        if event::poll(std::time::Duration::from_secs(1)).unwrap() {
            if let event::Event::Key(_) = event::read().unwrap() {
                break;
            }
        }
    }
    disable_raw_mode().unwrap();
}

fn clear_terminal() {
    Command::new("clear")
    .status()
    .expect("error");
}

fn wait_for_keypress() {
    loop {
        if let event::Event::Key(_) = event::read().unwrap() {
            break;
        }
    }
}

fn run_pomodoro(work_duration: u64, break_duration: u64) {
    loop {
        println!("Starting work session for {} minutes!", work_duration);
        start_timer(work_duration * 60);
        play_sound();

        println!("Starting break session for {} minutes!", break_duration);
        start_timer(break_duration * 60);
        play_sound();

        println!("Break session complete. Press any key to start a new work session.");
        wait_for_keypress();
        break;
    }
    clear_terminal();
}


fn print_timer(seconds: u64) {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    execute!(
        io::stdout(),
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine)
    ).unwrap();
    println!("Time remaining: {:02}:{:02}", minutes, seconds);
    io::stdout().flush().unwrap();
}

fn start_timer(seconds: u64) {
    let start = Instant::now();
    while Instant::now().duration_since(start).as_secs() < seconds {
        let remaining = seconds - Instant::now().duration_since(start).as_secs();
        print_timer(remaining);
        thread::sleep(Duration::from_secs(1));
    }
    clear_terminal();
}



fn main() {
    let mut counter: i8 = 1;
    let mut task_list : Vec<String> = Vec::new();

    clear_terminal();
    
    while counter != 0 {
        
        println!("--- Welcome to focusToRust ---");
        println!("\n1) save task\n2) task list\n3) start pomodoro \n4) configure pomodoro");
        println!("5) delete task \n6) exit \n");
        print!("select a option: ");
        io::stdout().flush().unwrap();
    
        let mut option= String::new();
        io::stdin().read_line(&mut option).expect("error");
        let option = option.trim();

        if option == "1" {
            
            print!("name of task: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("error"); 
            task_list.push(name);

            println!("\npress a key!!");
            wait_key();
            clear_terminal();

        } else if option == "2" {

            print!("\n");
            for task in task_list.iter() {
                print!("- {task}");
            }
            println!("\npress a key!!");
            wait_key();
            clear_terminal();

        } else if option == "3" {

            terminal::enable_raw_mode().unwrap();
            run_pomodoro(20, 5);
            terminal::disable_raw_mode().unwrap();

        } else if option == "4" {

            let mut selection = String::new();
            println!("How long do you want your pomodoro to last?");
            print!("time: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut selection).expect("error");
            
            match selection.trim().parse::<u64>() {
                Ok(num) => {
                    terminal::enable_raw_mode().unwrap();
                    run_pomodoro(num, 5);
                    terminal::disable_raw_mode().unwrap();
                }
                Err(_) => {
                    println!("¡Error! No es un número válido.");
                }
            };

        } else if option == "5" {
            
            print!("\n");
            for task in task_list.iter() {
                print!("- {task}");
            }
            print!("\nSelect a task to delete (use the index): ");
            io::stdout().flush().unwrap();
            
            let mut select = String::new();
            io::stdin().read_line(&mut select).expect("error");
            
            match select.trim().parse::<usize>() {
                Ok(num) => {
                    match task_list.get(num) {
                       Some(_) => {
                            task_list.remove(num);
                       },
                       None => {
                            print!("{num} is not index");
                       }
                    }
                }
                Err(_)  => {
                    print!("usize parse error");
                }
            };

            println!("\npress a key!!");
            wait_key();
            clear_terminal();

        } else if option == "6" {
            counter = 0;
        } else if option.to_lowercase() == "clear" {
            clear_terminal();
        } else {
            clear_terminal();
        }

    }
}
