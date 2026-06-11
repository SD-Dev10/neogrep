//use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::event::{self, KeyCode};
use layout::{Constraint, Flex, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};
use ratatui::widgets::{List, ListState};
use ratatui::{
    DefaultTerminal,
    style::{Color, Modifier, Style},
};
use std::path::PathBuf;
pub fn qpeek_w(
    mut terminal: DefaultTerminal,
    peek_vec: Vec<(PathBuf, String)>,
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

            frame.render_widget(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title("File List")
                    .border_type(BorderType::Rounded),
                layout[0],
            );
            frame.render_widget(
                Block::bordered()
                    .title_alignment(Alignment::Center)
                    .title("Matched Content")
                    .border_type(BorderType::Rounded),
                layout[1],
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

            let file_names: Vec<String> = peek_vec
                .iter()
                .map(|(fname, _)| fname.to_string_lossy().into_owned())
                .collect();

            let list = List::new(file_names)
                .style(Color::Rgb(130, 209, 115))
                .highlight_style(Modifier::REVERSED)
                .highlight_symbol("> ");

            frame.render_stateful_widget(list, fbox_vertical_center[0], &mut list_state);

            let content_index = list_state.selected();
            match content_index {
                Some(index) => {
                    let content_to_render = &peek_vec[index].1;
                    let paragraph = Paragraph::new(content_to_render.as_str())
                        .block(Block::default())
                        .wrap(Wrap { trim: true });
                    frame.render_widget(paragraph, cbox_vertical_center[0]);
                }
                None => println!("No index found"),
            }

            let info_span = Span::styled(
                "Press <q> to exit, use <j> and <k> to move Up/Down",
                Style::default().fg(Color::Rgb(0, 200, 180)),
            );
            let span_paragraph = Paragraph::new(info_span).alignment(Alignment::Center);

            // 2. Adjust the vertical layout constraints to lift the widget
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
