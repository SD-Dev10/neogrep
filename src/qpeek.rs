use crate::util::Peek;
use ansi_to_tui::IntoText as _;
use crossterm::event::{self, KeyCode};
use layout::{Constraint, Flex, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::widgets::{List, ListState};
use ratatui::{
    DefaultTerminal,
    style::{Color, Modifier, Style},
};

pub fn qpeek_w(
    mut terminal: DefaultTerminal,
    filtered_vec: Vec<Peek>,
    query: &str,
) -> color_eyre::Result<()> {
    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|frame| {
            let total_area = frame.area();

            frame.render_widget(
                Block::default().style(Style::default().bg(Color::Rgb(15, 10, 10))),
                total_area,
            );

            let horizontal_center = Layout::horizontal([Constraint::Percentage(80)])
                .flex(Flex::Center)
                .split(total_area);

            let smaller_box_area = Layout::vertical([Constraint::Percentage(80)])
                .flex(Flex::Center)
                .split(horizontal_center[0]);

            let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                .split(smaller_box_area[0]);

            let gray = Color::Rgb(105, 105, 105);

            frame.render_widget(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title(" File List ")
                    .border_type(BorderType::Rounded)
                    .title_style(Style::default().fg(gray)),
                layout[0],
            );

            let fbox_horizontal_center = Layout::horizontal([Constraint::Percentage(70)])
                .flex(Flex::Center)
                .split(layout[0]);
            let fbox_vertical_center = Layout::vertical([Constraint::Percentage(90)])
                .flex(Flex::Center)
                .split(fbox_horizontal_center[0]);

            frame.render_widget(Block::new(), fbox_vertical_center[0]);

            let cbox_horizontal_center = Layout::horizontal([Constraint::Percentage(70)])
                .flex(Flex::Center)
                .split(layout[1]);
            let cbox_vertical_center = Layout::vertical([Constraint::Percentage(90)])
                .flex(Flex::Center)
                .split(cbox_horizontal_center[0]);

            let file_names: Vec<String> = filtered_vec
                .iter()
                .filter_map(|peek| {
                    if let Some(name) = peek.file_name.as_deref() {
                        Some(name.display().to_string())
                    } else {
                        None
                    }
                })
                .collect();
            let list = List::new(file_names)
                .style(Color::Rgb(130, 209, 115))
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ");

            frame.render_stateful_widget(list, fbox_vertical_center[0], &mut list_state);

            let content_index = list_state.selected();

            //Dynamic title of matching content
            match content_index {
                Some(index) => {
                    let title_to_render = filtered_vec[index]
                        .file_name
                        .as_deref()
                        .map(|name| name.display().to_string())
                        .unwrap_or_else(|| " Untitled ".to_string());
                    let formatted_title = format!(" {} ", title_to_render);
                    frame.render_widget(
                        Block::bordered()
                            .title_alignment(Alignment::Center)
                            .title(formatted_title.as_str())
                            .title_style(Style::default().fg(gray))
                            .border_type(BorderType::Rounded),
                        layout[1],
                    );
                }
                None => {}
            }

            // Content window to check the matched highlighted word
            match content_index {
                Some(line_vector_idx) => {
                    let mut content_to_render = filtered_vec[line_vector_idx].content_vec.clone();
                    let colored_query = format!("\x1b[38;5;229m{}\x1b[0m", query);

                    for line in &mut content_to_render {
                        if line.contains(query) {
                            *line = line.replace(query, &colored_query);
                        }
                    }

                    let mut final_text = Text::default();

                    for (idx, line) in content_to_render.into_iter().enumerate() {
                        let line_num_str = format!("{}  ", idx + 1);
                        let mut spans = vec![Span::raw(line_num_str).fg(gray)];

                        let parsed_text = line.as_str().into_text().unwrap_or_default();
                        for t_line in parsed_text.lines {
                            let mut found_highlight = false;
                            let color = Color::Rgb(237, 234, 226);
                            for mut span in t_line.spans {
                                if span.style.fg == Some(Color::Indexed(229)) {
                                    found_highlight = true;
                                } else if found_highlight {
                                    span = span.fg(color);
                                } else if span.style.fg.is_none() {
                                    span = span.fg(color);
                                }

                                spans.push(span);
                            }
                        }
                        final_text.lines.push(Line::from(spans));
                    }

                    let paragraph = Paragraph::new(final_text).wrap(Wrap { trim: false });
                    frame.render_widget(paragraph, cbox_vertical_center[0]);
                }
                None => {}
            }

            //Info span to for keymaps
            let info_span = Span::styled(
                "Press <q> to exit, use <j> and <k> to move Up/Down",
                Style::default().fg(Color::Rgb(0, 200, 180)),
            );
            let span_paragraph = Paragraph::new(info_span).alignment(Alignment::Center);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(2),
                ])
                .split(horizontal_center[0]);

            frame.render_widget(span_paragraph, chunks[1]);
        })?;

        //Keymaps event handlers
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    }
}
