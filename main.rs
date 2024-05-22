mod instructions;

use std::env;

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

use instructions::registers;
use instructions::instruction_types::DecodedInst;
use instructions::InstFunc;

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
    frame.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .block(Block::bordered().title("risc-v-sim"))
            .white()
            .on_blue(),
            frame.size(),
    );
}

fn event_handler() -> io::Result<bool>{
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

/*
fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    registers::set(15, 15);
    //registers::print();

    let coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

    for coded in coded_instruction_arr{
        let decoded_inst: DecodedInst;
        match DecodedInst::decode(coded) {
            Ok(v) => decoded_inst = v,
            Err(_e) => break
        }
        //println!("{}", decoded_inst.to_string());
        let decoded_funct: i32 = decoded_inst.funct3;
        let instruction_function: InstFunc = instructions::I_CODES.get( &decoded_funct).cloned().expect("REASON");
        instruction_function(decoded_inst);
    }
}*/
