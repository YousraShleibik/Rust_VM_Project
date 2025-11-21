use std::time::{Duration, Instant};
use std::fmt::Debug;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

// Bring your project crate into scope
use rust_vm_project::{
    VirtualMachine, Chunk, OpCode, InterpretResult, Value, opcode_to_u8,
};

// ---------- VM adapter so the UI stays decoupled ----------
trait VmIntf {
    type Value: Debug;
    fn ip(&self) -> usize;
    fn stack(&self) -> &[Self::Value];
    fn chunk(&self) -> &Chunk;
    fn step(&mut self) -> InterpretResult;
    fn reset_with(&mut self, chunk: Chunk);
}

impl VmIntf for VirtualMachine {
    type Value = Value;

    fn ip(&self) -> usize {
        // Your VM stores ip (likely usize/u16/u32). Adjust cast if needed.
        self.ip as usize
    }

    fn stack(&self) -> &[Self::Value] {
        &self.stack
    }

    fn chunk(&self) -> &Chunk {
        // Your VM stores Option<Chunk>
        self.chunk.as_ref().expect("VM has no loaded chunk")
    }

    fn step(&mut self) -> InterpretResult {
        // Call your one-instruction function. If named differently,
        // change here (e.g., self.execute_one(), self.run_instruction(), etc.)
        self.run()
    
    }

    fn reset_with(&mut self, chunk: Chunk) {
    self.chunk = Some(chunk);
    self.ip = 0;
    self.stack.clear();
    self.globals.clear();
    }


}

// ---------- App state ----------
struct App<V: VmIntf> {
    vm: V,
    running: bool,
    // we’ll rebuild the same demo chunk on reset to avoid requiring Clone
}

impl<V: VmIntf> App<V> {
    fn new(vm: V) -> Self {
        Self { vm, running: false }
        
    }
}

// ---------- Demo program for the UI ----------
fn build_demo_chunk() -> Chunk {
    let mut chunk = Chunk::init_chunk();

    // Add constants (adjust Value variant name if needed)
    // use integers that fit your Number = i16
    let c0 = chunk.add_constant(Value::ValNumber(1));
    let c1 = chunk.add_constant(Value::ValNumber(2));


    // OpConstant <idx>
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 1);
    chunk.write_to_chunk(c0 as u8, 1);

    chunk.write_to_chunk(opcode_to_u8(OpCode::OpConstant), 1);
    chunk.write_to_chunk(c1 as u8, 1);

    // Add
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpAdd), 1);

    // Return
    chunk.write_to_chunk(opcode_to_u8(OpCode::OpReturn), 1);

    chunk
}

fn main() -> anyhow::Result<()> {
    // Build a small demo program and boot the VM
    let demo = build_demo_chunk();

    let mut vm = VirtualMachine::init_machine();
    vm.reset_with(demo);

    // Terminal setup and run
    let mut terminal = ratatui::init();
    let res = run_app(&mut terminal, App::new(vm));
    ratatui::restore();
    res
}

fn run_app<V: VmIntf>(terminal: &mut DefaultTerminal, mut app: App<V>) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(1000 / 30); // ~30 fps

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Input
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Right => {
                            let _ = app.vm.step();
                        }
                        KeyCode::Char(' ') => {
                            app.running = !app.running;
                        }
                        KeyCode::Char('r') => {
                            // Recreate the same demo chunk and reset the VM
                            app.vm.reset_with(build_demo_chunk());
                        }
                        _ => {}
                    }
                }
            }
        }

        // Run-fast mode
        if app.running && last_tick.elapsed() >= Duration::from_millis(1) {
            let _ = app.vm.step();
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    Ok(())
}

// ---------- UI drawing ----------
fn ui<V: VmIntf>(f: &mut Frame, app: &App<V>) {
    let area = f.area();

    // Vertical split: top (bytecode + stack) / bottom (status)
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);
    let top = v[0];
    let bottom = v[1];

    // Horizontal split for top: left (bytecode) / right (stack)
    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(top);
    let left = h[0];
    let right = h[1];

    draw_chunk(f, left, app);
    draw_stack(f, right, app);
    draw_status_bar(f, bottom, app);
}

fn draw_chunk<V: VmIntf>(f: &mut Frame, area: Rect, app: &App<V>) {
    let ip = app.vm.ip();

    // REQUIREMENT: Chunk must expose `code()` → &[u8]
    // Add in your lib.rs:
    //   impl Chunk {
    //       pub fn code(&self) -> &[u8] { &self.code }
    //   }
    //let bytes = app.vm.chunk().code();
    let bytes: &[u8] = &[]; // placeholder, no bytecode view

    let mut lines: Vec<Line> = Vec::new();
    let mut i = 0usize;

    // Disassemble linear bytecode (simple view)
    while i < bytes.len() {
        let op = bytes[i];
        let marker = if i == ip { "▶" } else { " " };

        let name = mnemonic_byte(op);
        let line = Line::from(vec![
            Span::styled(format!("{marker} {:04} ", i), Style::default().fg(Color::DarkGray)),
            Span::raw(name),
        ]);

        if i == ip {
            lines.push(line.clone().style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
        } else {
            lines.push(line);
        }

        // Advance i by instruction width
        // For OpConstant, there’s one operand byte (index). Adjust if your encoding differs.
        if op == opcode_to_u8(OpCode::OpConstant) {
            i += 2; // opcode + 1 operand
        } else {
            i += 1; // single-byte op
        }
    }

    if lines.is_empty() {
        lines.push(Line::from("(no bytecode loaded)"));
    }

    let block = Block::default().title("Bytecode").borders(Borders::ALL);
    let para = Paragraph::new(Text::from(lines)).block(block).wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn draw_stack<V: VmIntf>(f: &mut Frame, area: Rect, app: &App<V>) {
    let stack = app.vm.stack();
    let mut lines: Vec<Line> = Vec::new();
    if stack.is_empty() {
        lines.push(Line::from("(empty)"));
    } else {
        for (i, val) in stack.iter().enumerate() {
            // Value doesn’t implement Display, so use Debug
            lines.push(Line::from(format!("{:>3}: {:?}", i, val)));
        }
    }
    let block = Block::default().title("Stack (bottom → top)").borders(Borders::ALL);
    let para = Paragraph::new(Text::from(lines)).block(block);
    f.render_widget(para, area);
}

fn draw_status_bar<V: VmIntf>(f: &mut Frame, area: Rect, app: &App<V>) {
    let ip = app.vm.ip();
    let run_state = if app.running { "RUNNING (Space to pause)" } else { "PAUSED (Space to run)" };
    let help = "Keys: → step   Space run/pause   r reset   q quit";

    let text = Text::from(vec![
        Line::from(format!("IP: {ip}   {run_state}")),
        Line::from(help),
    ]);

    let block = Block::default().borders(Borders::ALL).title("Status");
    let para = Paragraph::new(text).block(block);
    f.render_widget(para, area);
}

// Map an opcode byte to a short mnemonic
fn mnemonic_byte(op: u8) -> &'static str {
    if op == opcode_to_u8(OpCode::OpConstant) { "CONSTANT" }
    else if op == opcode_to_u8(OpCode::OpAdd) { "ADD" }
    else if op == opcode_to_u8(OpCode::OpReturn) { "RET" }
    else { "OP?" }
}
