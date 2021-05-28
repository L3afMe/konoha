use std::{
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crossterm::event::{self, Event as CTEvent, KeyEvent, MouseEvent};

use super::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
}

pub fn handle_event(receiver: &Receiver<Event>, app: &mut App) {
    if let Ok(event) = receiver.recv_timeout(Duration::ZERO) {
        match event {
            Event::Key(key) => app.on_key_press(key),
            Event::Mouse(event) => app.on_mouse(event),
            Event::Tick => app.on_tick(),
        }
    }
}

pub struct EventReceiver {
    pub key_handle:  JoinHandle<()>,
    pub tick_handle: JoinHandle<()>,
    pub receiver:    Receiver<Event>,
}

pub fn spawn_event_listener(tick_ms: u64) -> EventReceiver {
    let (sr, receiver) = mpsc::channel();

    let key_handle = {
        let sr = sr.clone();
        thread::spawn(move || {
            let tick_rate = Duration::from_millis(tick_ms);
            let mut last_tick = Instant::now();

            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_millis(0));

                if event::poll(timeout).unwrap_or_default() {
                    if let Ok(event) = event::read() {
                        let term_event = match event {
                            CTEvent::Key(event) => Event::Key(event),
                            CTEvent::Mouse(event) => Event::Mouse(event),
                            CTEvent::Resize(..) => continue,
                        };

                        if let Err(_why) = sr.send(term_event) {
                            // Handle why
                        }
                    }
                }
                if last_tick.elapsed() >= tick_rate {
                    last_tick = Instant::now();
                }
            }
        })
    };

    let tick_rate = Duration::from_millis(tick_ms);
    let tick_handle = {
        thread::spawn(move || loop {
            if let Err(_why) = sr.send(Event::Tick) {
                // Handle why
            }
            thread::sleep(tick_rate)
        })
    };

    EventReceiver {
        key_handle,
        tick_handle,
        receiver,
    }
}
