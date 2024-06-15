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
const coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

fn main() -> Result<()> {
    //SIM Init
    registers::set(15, 15);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let layout_arr: [Layout; 2];
    let frame_arr: [Frame; 3];

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
        Paragraph::new("Next decoded instruction goes here.")
            .block(Block::bordered().title("Next instruction."))
            .white()
            .on_blue(),
            inner_layout[0]
    );
    frame.render_widget(
        Paragraph::new(registers::print())
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
                    let _ = step_sim();
                    /*
                    let decoded_inst: DecodedInst;
                    match DecodedInst::decode(coded) {
                        Ok(v) => decoded_inst = v,
                        Err(_e) => return Ok(true)
                    }
                    //println!("{}", decoded_inst.to_string());
                    let decoded_funct: i32 = decoded_inst.funct3;
                    let instruction_function: InstFunc = instructions::I_CODES.get( &decoded_funct).cloned().expect("REASON");
                    instruction_function(decoded_inst);
                    */
                }
                else if key.code == KeyCode::Char('r')
                {
                    reset_sim();
                }
            }
        }
    }
    return Ok(false);
}


fn step_sim() -> io::Result<bool>{
    // Fetch the next instruction
    let fetched_instruction = coded_instruction_arr[registers::get(registers::PC_INDEX) as usize];
    
    // Decode it
    let decoded_inst: DecodedInst;
    match DecodedInst::decode(fetched_instruction) {
        Ok(v) => decoded_inst = v,
        Err(_e) => return Ok(false)
    }
    let decoded_funct: i32 = decoded_inst.funct3;
    let instruction_function: InstFunc = instructions::I_CODES.get(&decoded_funct).cloned().expect("REASON");
    
    // Execute it
    instruction_function(decoded_inst);
    
    // Update program counter
    registers::set(registers::PC_INDEX, registers::get(registers::PC_INDEX) + 1);
    return Ok(true);
}

fn reset_sim(){
    registers::reset();
}
/*
fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    registers::set(15, 15);
    //registers::print();

    let coded_instruction_arr: [i32; 9] = [0x00730293, 0x00734293, 0x00736293, 0x00737293, 0x00731293, 0x00735293, 0x40735293, 0x00732293, 0x00733293];

    for coded in coded_instruction_arr{

    }
}*/
