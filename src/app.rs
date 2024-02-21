use crossterm::{
    event::{self, DisableMouseCapture, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io,
    sync::mpsc,
    thread::{self},
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod stow;
mod ui;

enum Event<I> {
    Input(I),
    Tick,
}

pub struct App {
    current_dir: String,
    config_dir_popup: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_dir: String::new(),
            config_dir_popup: false,
        }
    }
}

fn run_input_thread() -> mpsc::Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    rx
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let rx = run_input_thread();
    loop {
        terminal.draw(ui::render)?;
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            },
            Event::Tick => {}
        }
    }
}
