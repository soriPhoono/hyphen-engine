mod util;

use std::{
    io::{self, Write},
    thread,
    time::{self, Duration, Instant},
};

use crossterm::{
    cursor::{
        Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveToNextLine, MoveToPreviousLine, MoveUp,
        Show,
    },
    event::{poll, read, Event, KeyCode},
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, BeginSynchronizedUpdate, Clear, ClearType,
        DisableLineWrap, EnableLineWrap, EndSynchronizedUpdate, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use util::FPSTimer;

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut fps_timer = FPSTimer::new(60);

    loop {
        fps_timer.reset();

        queue!(stdout, BeginSynchronizedUpdate)?; // Begin a synchronized update
        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?; // Clear the screen

        // render here

        queue!(stdout, EndSynchronizedUpdate)?; // End the synchronized update

        stdout.flush()?; // Make the changes visible

        // update here
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    // Handle key events here
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        fps_timer.wait();
    }

    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;

    Ok(())
}
