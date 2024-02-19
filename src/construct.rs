use anyhow::{Context as _, Result};

/// Makes a markup for the full terminal window.
#[must_use]
pub fn markup_chunks(frame_size: tui::layout::Rect) -> Vec<tui::layout::Rect> {
	tui::layout::Layout::default()
		.margin(2)
		.constraints(vec![
			tui::layout::Constraint::Length(3),
			tui::layout::Constraint::Min(2),
			tui::layout::Constraint::Length(3),
		])
		.split(frame_size)
}

/// Makes a markup for a notes list paragraph.
/// The list paragraph is the second chunk of the main markup.
///
/// Partitions the second chunk into two more parts: List with note
/// IDs and details of the selected note.
#[must_use]
pub fn markup_list_paragraph_chunks(
	frame_size: tui::layout::Rect,
) -> Vec<tui::layout::Rect> {
	tui::layout::Layout::default()
		.direction(tui::layout::Direction::Horizontal)
		.constraints(vec![
			tui::layout::Constraint::Percentage(5),
			tui::layout::Constraint::Percentage(95),
		])
		.split(frame_size)
}

/// Makes tabs with action cues. Intended for display on the first chunk.
#[must_use]
pub fn make_tabs<'a>() -> tui::widgets::Tabs<'a> {
	let menu: Vec<tui::text::Spans> = crate::consts::TAB_TITLES
		.iter()
		.map(|t| {
			let (first_letter, rest_of_word) = t.split_at(1);
			let first_letter_style = tui::style::Style::default()
				.fg(tui::style::Color::Yellow)
				.add_modifier(tui::style::Modifier::UNDERLINED);
			let rest_of_word_style =
				tui::style::Style::default().fg(tui::style::Color::White);
			tui::text::Spans(vec![
				tui::text::Span::styled(first_letter, first_letter_style),
				tui::text::Span::styled(rest_of_word, rest_of_word_style),
			])
		})
		.collect();
	let style = tui::style::Style::default().fg(tui::style::Color::White);
	let block = tui::widgets::Block::default()
		.title(crate::consts::TABS_PARAGRAPH_TITLE)
		.borders(tui::widgets::Borders::ALL);

	tui::widgets::Tabs::new(menu)
		.divider(tui::text::Span::raw("|"))
		.style(style)
		.block(block)
}

/// Creates a paragraph to display. Designed to be displayed on the second
/// chunk, which is divided into two more parts: Note ID list and table with
/// details.
pub fn make_list_paragraph<'a>(
	notes_state: &tui::widgets::ListState,
) -> Result<(tui::widgets::List<'a>, tui::widgets::Table<'a>)> {
	let notes =
		crate::note::Note::get_all().context("Failed to get all notes.")?;
	let selected_note =
		notes.get(notes_state.selected().unwrap()).unwrap().clone();

	let list = {
		let items: Vec<tui::widgets::ListItem> = notes
			.iter()
			.enumerate()
			.map(|(i, _)| {
				let span = tui::text::Span::raw(i.to_string());
				tui::widgets::ListItem::new(tui::text::Spans(vec![span]))
			})
			.collect();
		let block = tui::widgets::Block::default()
			.title(crate::consts::LIST_PARAGRAPH_TITLE)
			.borders(tui::widgets::Borders::ALL)
			.style(tui::style::Style::default().fg(tui::style::Color::White));
		let highlight_style = tui::style::Style::default()
			.bg(tui::style::Color::Yellow)
			.fg(tui::style::Color::Black)
			.add_modifier(tui::style::Modifier::BOLD);
		tui::widgets::List::new(items)
			.block(block)
			.highlight_style(highlight_style)
	};
	let details_table = {
		let block = tui::widgets::Block::default()
			.title(crate::consts::LIST_PARAGRAPH_DETAILS_TITLE)
			.borders(tui::widgets::Borders::ALL)
			.style(tui::style::Style::default().fg(tui::style::Color::White));
		let header = tui::widgets::Row::new(
			crate::consts::LIST_PARAGRAPH_DETAILS_COLUMN_TITLES,
		);
		let content = vec![tui::widgets::Row::new(vec![
			tui::widgets::Cell::from(tui::text::Span::raw(
				selected_note.text.clone(),
			)),
			tui::widgets::Cell::from(tui::text::Span::raw(
				crate::helpers::beautify_datetime(selected_note.created_at),
			)),
		])];
		tui::widgets::Table::new(content).header(header).block(block).widths(
			&[
				tui::layout::Constraint::Percentage(85),
				tui::layout::Constraint::Percentage(15),
			],
		)
	};

	Ok((list, details_table))
}

/// If there are no notes at the moment, we can insert this paragraph in the
/// second chunk.
#[must_use]
pub fn make_empty_list_paragraph<'a>() -> tui::widgets::Paragraph<'a> {
	let block = tui::widgets::Block::default()
		.title(crate::consts::INFO_PARAGRAPH_TITLE)
		.borders(tui::widgets::Borders::ALL)
		.style(tui::style::Style::default().fg(tui::style::Color::White));
	let style = tui::style::Style::default().fg(tui::style::Color::LightRed);
	tui::widgets::Paragraph::new(crate::consts::EMPTY_LIST_PARAGRAPH_TEXT)
		.block(block)
		.style(style)
		.alignment(tui::layout::Alignment::Center)
}

/// Creates a paragraph with additional information.
/// Intended to be displayed in the third chunk.
pub fn make_info_paragraph<'a>() -> Result<tui::widgets::Paragraph<'a>> {
	let block = tui::widgets::Block::default()
		.title(crate::consts::INFO_PARAGRAPH_TITLE)
		.borders(tui::widgets::Borders::ALL)
		.style(tui::style::Style::default().fg(tui::style::Color::White));
	let style = tui::style::Style::default().fg(tui::style::Color::LightCyan);
	let count = crate::note::Note::get_count()
		.context("Failed to get notes count.")?;
	Ok(tui::widgets::Paragraph::new(format!("Notes count: {count}"))
		.block(block)
		.style(style)
		.alignment(tui::layout::Alignment::Center))
}

/// Creates a paragraph for data input when creating notes.
/// Designed to replace the paragraph with additional information
/// created with `make_info_paragraph` in the third chunk.
#[must_use]
pub fn make_create_paragraph(input: &str) -> tui::widgets::Paragraph {
	let block = tui::widgets::Block::default()
		.title(crate::consts::CREATE_PARAGRAPH_TITLE)
		.borders(tui::widgets::Borders::ALL);
	tui::widgets::Paragraph::new(input).block(block)
}

/// Creates a paragraph for data input when updating notes.
/// Designed to replace the paragraph with additional information
/// created with `make_info_paragraph` in the third chunk.
#[must_use]
pub fn make_update_paragraph(input: &str) -> tui::widgets::Paragraph {
	let block = tui::widgets::Block::default()
		.title(crate::consts::UPDATE_PARAGRAPH_TITLE)
		.borders(tui::widgets::Borders::ALL);
	let input = tui::text::Span::styled(
		input,
		tui::style::Style::default().fg(tui::style::Color::Yellow),
	);
	tui::widgets::Paragraph::new(input).block(block)
}
