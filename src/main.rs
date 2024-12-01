use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use page_rank_from_scratch::{crawler::WebCrawler, page_rank::PageRanker};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::sync::{Arc, Mutex};
use std::{io, rc::Rc};

pub enum Mode {
    Normal,
    Insert,
}

fn main() -> io::Result<()> {
    let page_registry = WebCrawler::load("100_sites_with_roots.json")
        .expect("Failed to load page registry from file");
    let pageranker = Arc::new(Mutex::new(PageRanker::from_registry(page_registry)));

    let title_card = r#"
  _                       _      
 | |                     | |     
 | |     ___  _   _  __ _| | ___ 
 | |    / _ \| | | |/ _` | |/ _ \
 | |___| (_) | |_| | (_| | |  __/
 |______\___/ \__,_|\__, |_|\___|
                     __/ |       
                    |___/        "#;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut search_results: Vec<(String, String)> = vec![];
    let mut selected = 0;
    let mut mode = Mode::Normal;

    loop {
        terminal.draw(|frame| {
            let size = frame.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(9), // Title
                        Constraint::Length(3), // Input
                        Constraint::Min(1),    // Results
                    ]
                    .as_ref(),
                )
                .split(size);

            let input_box = Paragraph::new(input.clone())
                .block(Block::default().borders(Borders::ALL).title("Search"));
            frame.render_widget(input_box, chunks[1]);

            let title = Paragraph::new(title_card)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            frame.render_widget(title, chunks[0]);

            let result_area = chunks[2];
            let visible_results = 12;
            let total_results = search_results.len();

            let mut start = 0;
            let mut end = visible_results;

            if selected >= visible_results {
                end = selected + 1;
                start = end - visible_results;
            }

            end = end.min(total_results);

            let results_to_display = &search_results[start..end];

            let result_chunks = create_fixed_chunks(visible_results, result_area);

            for (i, (title, url)) in results_to_display.iter().enumerate() {
                let box_content = format!("{}\n{}", title, url);

                let color = if i + start == selected {
                    Color::Blue
                } else {
                    Color::Yellow
                };

                let result_box = Paragraph::new(Text::from(box_content)).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Result")
                        .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                );
                frame.render_widget(result_box, result_chunks[i]);
            }
        })?;

        // Handle events
        if let Event::Key(key) = event::read()? {
            match mode {
                Mode::Insert => match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        if let Some(rankings) = pageranker.lock().unwrap().search(input.trim()) {
                            search_results = rankings
                                .iter()
                                .map(|site| (site.title.clone(), site.url.clone()))
                                .collect();
                            selected = 0;
                        } else {
                            search_results = vec![(
                                "No results found".to_string(),
                                "Try a different query.".to_string(),
                            )];
                        }
                        input.clear();
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected < search_results.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Esc => mode = Mode::Normal,
                    _ => {}
                },
                Mode::Normal => match key.code {
                    KeyCode::Char(c) => match c {
                        'q' => break,
                        'j' => {
                            if selected < search_results.len() {
                                selected += 1;
                            }
                        }
                        'k' => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        'i' => mode = Mode::Insert,
                        _ => {}
                    },
                    KeyCode::Enter => {
                        let url = &search_results[selected].1;
                        open::that(url).expect("Failed to open");
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}

/// Creates fixed chunks for a fixed number of results
fn create_fixed_chunks(visible_results: usize, area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(5); visible_results])
        .split(area)
}
