//RATAUI CODE
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::*
};
use std::io::{self, stdout, Result};
//END RATATUI CODE

mod simulator;

fn main() -> Result<()> {

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    terminal.clear()?;
    // TODO main loop
    let mut should_quit = false;
    
    while !should_quit{
        //draw the UI
        terminal.draw(ui)?;
        //TODO: handle events
        should_quit = event_handler()?
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame){
    let outer_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(frame.size());

    let inner_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(15),
        Constraint::Percentage(85),
    ])
    .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("Welcom to risc-v simulator! Press:\n - 's' to step to the next instruction \n - 'r' to reset the sim (set all registers back to 0). \n - 'q' to quit.")
            .block(Block::bordered().title("risc-v-sim"))
            .white()
            .on_blue(),
            outer_layout[0]
    );
    frame.render_widget(
        Paragraph::new("Last decoded instruction goes here.")
            .block(Block::bordered().title("Last instruction."))
            .white()
            .on_blue(),
            inner_layout[0]
    );
    frame.render_widget(
        Paragraph::new(simulator::instructions::registers::print())
            .block(Block::bordered().title("Registers"))
            .white()
            .on_blue(),
            inner_layout[1]
    );
}

fn event_handler() -> io::Result<bool>{
    // 16 ms ~= 60 fps
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
            {
                if key.code == KeyCode::Char('q')
                {
                    return Ok(true);
                }else if key.code == KeyCode::Char('s')
                {
                    let _ = simulator::step();

                }
                else if key.code == KeyCode::Char('r')
                {
                    simulator::reset();
                }
            }
        }
    }
    return Ok(false);
}
