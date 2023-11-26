use crossterm::{terminal, execute};
use std::io::Result;

fn startup() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(std::io::stderr(), terminal::EnterAlternateScreen)?;

    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), terminal::EnterAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}