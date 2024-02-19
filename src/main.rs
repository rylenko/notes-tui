mod construct;
mod consts;
mod helpers;
mod note;
mod state;

use anyhow::{Context as _, Result};

fn delete_selected_note(
	notes_state: &mut tui::widgets::ListState,
) -> Result<()> {
	let index = notes_state.selected().unwrap();
	note::Note::delete(index).context("Failed to delete a node.")?;

	let new_selected_index = if index == 0 { 0 } else { index - 1 };
	notes_state.select(Some(new_selected_index));
	Ok(())
}

fn main() -> Result<()> {
	use termion::{input::TermRead as _, raw::IntoRawMode as _};

	helpers::create_database_if_not_exists()
		.context("Failed to create database if not exists.")?;
	let (tx, rx): (
		std::sync::mpsc::Sender<termion::event::Event>,
		std::sync::mpsc::Receiver<termion::event::Event>,
	) = std::sync::mpsc::channel();

	// Starting a thread that will process and send us keystrokes
	std::thread::spawn(move || {
		for event in std::io::stdin().events() {
			tx.send(event.unwrap()).unwrap();
		}
	});

	// Initializing the terminal
	let stdout = std::io::stdout()
		.into_raw_mode()
		.context("Failed to use raw mode.")?;
	let backend = tui::backend::TermionBackend::new(stdout);
	let mut terminal = tui::Terminal::new(backend)
		.context("Failed to make a new terminal.")?;
	terminal.clear().context("Failed to clear a terminal.")?;

	let mut state = state::State::new();
	loop {
		terminal
			.draw(|frame| {
				let chunks = construct::markup_chunks(frame.size());

				// Render tabs
				let tabs = construct::make_tabs();
				frame.render_widget(tabs, chunks[0]);

				if note::Note::get_count().unwrap() == 0 {
					// Render empty list
					let widget = construct::make_empty_list_paragraph();
					frame.render_widget(widget, chunks[1]);
				} else {
					// Render list with identifiers and table with details
					let list_chunks =
						construct::markup_list_paragraph_chunks(chunks[1]);
					let (left, right) =
						construct::make_list_paragraph(&state.notes_state)
							.unwrap();
					frame.render_stateful_widget(
						left,
						list_chunks[0],
						&mut state.notes_state,
					);
					frame.render_widget(right, list_chunks[1]);
				}

				// Render third (bottom) widget
				let bottom_widget = match state.input_mode {
					helpers::InputMode::Null => {
						construct::make_info_paragraph().unwrap()
					}
					helpers::InputMode::Creating => {
						construct::make_create_paragraph(&state.input)
					}
					helpers::InputMode::Updating => {
						construct::make_update_paragraph(&state.input)
					}
				};
				frame.render_widget(bottom_widget, chunks[2]);

				match state.input_mode {
					helpers::InputMode::Null => {}
					helpers::InputMode::Creating
					| helpers::InputMode::Updating => {
						// Setting the cursor to the right of the text
						let x = chunks[2].x + 1 + state.input.len() as u16;
						let y = chunks[2].y + 1;
						frame.set_cursor(x, y);
					}
				}
			})
			.context("Failed to draw.")?;

		let key = match rx.recv().unwrap() {
			termion::event::Event::Key(k) => k,
			_ => continue,
		};

		match state.input_mode {
			helpers::InputMode::Null => match key {
				// Handling keystrokes when we don't input anything
				termion::event::Key::Char('q') => break,
				termion::event::Key::Char('c') => {
					state.input_mode = helpers::InputMode::Creating
				}
				termion::event::Key::Char('u')
					if note::Note::get_count()
						.context("Failed to get notes count.")?
						> 0 =>
				{
					let index = state.notes_state.selected().unwrap();
					state.input = note::Note::get(index)
						.context("Failed to get note.")?
						.text;
					state.input_mode = helpers::InputMode::Updating;
				}
				termion::event::Key::Char('d')
					if note::Note::get_count()
						.context("Failed to get notes count.")?
						> 0 =>
				{
					delete_selected_note(&mut state.notes_state)
						.context("Failed to delete a selected note.")?;
				}
				termion::event::Key::Char('k')
					if note::Note::get_count()
						.context("Failed to get notes count.")?
						> 0 =>
				{
					state
						.select_previous_note()
						.context("Failed to select previous note.")?
				}
				termion::event::Key::Char('j')
					if note::Note::get_count()
						.context("Failed to get notes count.")?
						> 0 =>
				{
					state
						.select_next_note()
						.context("Failed to select next note.")?
				}
				_ => {}
			},
			helpers::InputMode::Creating => match key {
				// Handling keystrokes when we input data to create a note
				termion::event::Key::Char('\n') => {
					note::Note::create(state.take_input())
						.context("Failed to create a note.")?;
					state.input_mode = helpers::InputMode::Null;
				}
				termion::event::Key::Esc => {
					state.input_mode = helpers::InputMode::Null
				}
				termion::event::Key::Backspace => state.pop_input(),
				termion::event::Key::Char(c) => state.write_input(c),
				_ => {}
			},
			helpers::InputMode::Updating => match key {
				// Handling keystrokes when we input data to update a note
				termion::event::Key::Char('\n') => {
					let index = state.notes_state.selected().unwrap();
					note::Note::update(index, state.take_input())
						.context("Failed to update a note.")?;
					state.input_mode = helpers::InputMode::Null;
				}
				termion::event::Key::Esc => {
					state.input_mode = helpers::InputMode::Null
				}
				termion::event::Key::Backspace => state.pop_input(),
				termion::event::Key::Char(c) => state.write_input(c),
				_ => {}
			},
		};
	}

	terminal.clear().context("Failed to clear a terminal.")?;
	Ok(())
}
