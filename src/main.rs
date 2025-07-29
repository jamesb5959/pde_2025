use std::{io, fs::OpenOptions, io::Write};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

// Quiz question with prompt, hint, and expected answer
struct Question {
    prompt: &'static str,
    hint: &'static str,
    answer: &'static str,
}

enum InputMode {
    Normal,
    Insert,
}

struct App {
    questions: Vec<Question>,
    current: usize,
    input: String,
    history: Vec<Spans<'static>>,
    input_mode: InputMode,
    scroll: u16,
}

impl App {
    fn new() -> Self {
        let questions = vec![
            Question {
                prompt: "1. What command starts VirtualBox in the debug build?", 
                hint: "Hint: From the debug/bin directory, run ./VirtualBox", 
                answer: "./VirtualBox",
            },
            Question {
                prompt: "2. How do you attach gdb to the VM process?", 
                hint: "Hint: sudo gdb -p $(pidof VirtualBoxVM)", 
                answer: "sudo gdb -p $(pidof VirtualBoxVM)",
            },
            Question {
                prompt: "3. Which GDB directive ignores SIGTRAP?", 
                hint: "Hint: handle SIGTRAP nostop noprint nopass", 
                answer: "handle SIGTRAP nostop noprint nopass",
            },
            Question {
                prompt: "4. How set the solib-search-path in GDB?", 
                hint: "Hint: set solib-search-path /home/pde/.../debug/bin:/home/pde/.../debug/bin/components", 
                answer: "set solib-search-path /home/pde/Downloads/VirtualBox-7.0.10/out/linux.amd64/debug/bin:/home/pde/Downloads/VirtualBox-7.0.10/out/linux.amd64/debug/bin/components",
            },
            Question {
                prompt: "5. What breakpoint filters VLAN IDs >=4096?", 
                hint: "Hint: break virtioNetR3CtrlVlan if uVlanId >= 4096", 
                answer: "break virtioNetR3CtrlVlan if uVlanId >= 4096",
            },
            Question {
                prompt: "6. How do you continue execution in GDB?", 
                hint: "Hint: c", 
                answer: "c",
            },
            Question {
                prompt: "7. Which GDB command shows locals?", 
                hint: "Hint: info locals", 
                answer: "info locals",
            },
            Question {
                prompt: "8. How remove the virtio-net module in the VM?", 
                hint: "Hint: sudo rmmod virtio-net", 
                answer: "sudo rmmod virtio-net",
            },
            Question {
                prompt: "9. How insert the exploit module exploit.ko?", 
                hint: "Hint: sudo insmod exploit.ko", 
                answer: "sudo insmod exploit.ko",
            },
            Question {
                prompt: "10. What do you expect after loading exploit.ko?", 
                hint: "Hint: The GDB breakpoint at virtioNetR3CtrlVlan should hit and info locals shows uVlanId>=4096", 
                answer: "breakpoint hit and info locals",
            },
        ];
        let initial = Spans::from(Span::styled(
            questions[0].prompt,
            Style::default().fg(Color::Magenta),
        ));
        Self {
            questions,
            current: 0,
            input: String::new(),
            history: vec![initial],
            input_mode: InputMode::Insert,
            scroll: 0,
        }
    }

    fn process_input(&mut self) {
        let trimmed = self.input.trim();
        if trimmed.is_empty() {
            return;
        }
        let q = &self.questions[self.current];
        if trimmed.eq_ignore_ascii_case("hint") {
            self.history.push(Spans::from(Span::raw(q.hint)));
        } else if trimmed == q.answer {
            self.history.push(Spans::from(Span::styled(
                "Correct!",
                Style::default().fg(Color::Green),
            )));
            self.current += 1;
            if self.current < self.questions.len() {
                let next = self.questions[self.current].prompt;
                self.history.push(Spans::from(Span::styled(
                    next,
                    Style::default().fg(Color::Magenta),
                )));
            } else {
                self.history.push(Spans::from(Span::styled(
                    "Quiz complete!",
                    Style::default().fg(Color::Yellow),
                )));
            }
        } else {
            self.history.push(Spans::from(Span::styled(
                "Incorrect, try again or type 'hint'",
                Style::default().fg(Color::Red),
            )));
        }
        append_to_file(&self.input);
        self.input.clear();
        self.scroll = 0;
    }
}

fn append_to_file(line: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("quiz_history.txt") {
        let _ = writeln!(f, "{}", line);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    if let Err(e) = res {
        eprintln!("Error: {:?}", e);
    }
    Ok(())
}

fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(size);

            let history = Paragraph::new(app.history.clone())
                .block(Block::default().title("Quiz").borders(Borders::ALL))
                .scroll((app.scroll, 0));
            f.render_widget(history, chunks[0]);

            let input = Paragraph::new(app.input.as_ref())
                .block(Block::default().title("Answer or 'hint'").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(input, chunks[1]);
            f.set_cursor(
                chunks[1].x + app.input.len() as u16 + 1,
                chunks[1].y + 1,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Enter => { app.process_input(); }
                    KeyCode::Up => { if app.scroll > 0 { app.scroll -= 1; } }
                    KeyCode::Down => { app.scroll += 1; }
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

