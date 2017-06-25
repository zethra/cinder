extern crate serial;
extern crate rustbox;
extern crate cinder;

use std::str;
use std::env;
use std::time::Duration;
use std::process::exit;
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

use serial::prelude::*;
use rustbox::Key;
use cinder::Screen;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    let port_str = env::args_os().nth(1).unwrap();
    let (input_tx, input_rx) = channel::<String>();
    let (output_tx, output_rx) = channel::<String>();

    thread::spawn(move || {
        let mut port = serial::open(&port_str).unwrap();
        interact(&mut port, input_rx, output_tx).unwrap();
    });
    
    let mut screen = Screen::new();
    screen.update();
    loop {
        if let Ok(msg) = output_rx.try_recv() {
            screen.write_output(msg);
        }
        match screen.rustbox.peek_event(Duration::from_millis(0), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(c) => screen.write_input(c),
                    Key::Backspace => screen.delete(),
                    Key::Esc => exit(0),
                    Key::Enter => input_tx.send(screen.enter()).unwrap(),
                    Key::Up => screen.history_up(),
                    Key::Down => screen.history_down(),
                    _ => {}
                }
            }
            Err(_) => {},
            _ => {}
        }
        screen.update();
    }
}

fn interact<T: SerialPort>(port: &mut T, rx: Receiver<String>, tx: Sender<String>) -> serial::Result<()> {
    try!(port.configure(&SETTINGS));
    try!(port.set_timeout(Duration::from_secs(1)));

    let mut buf = [0; 100];

    loop {
        if let Ok(msg) = rx.try_recv() {
            try!(port.write(&msg.into_bytes()));
            try!(port.write(&[10]));
        }
        if let Ok(_) = port.read(&mut buf[..]) {
            if let Ok(msg) = str::from_utf8(&buf) {
                tx.send(msg.trim_matches('\0').to_string()).unwrap();
            } else {
                panic!("Failed to parse output.");
            }
            buf = [0; 100];
        }
    }
}
