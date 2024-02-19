use anyhow::{Context as _, Result};

pub struct State {
	pub notes_state: tui::widgets::ListState,
	pub input_mode: crate::helpers::InputMode,
	pub input: String,
}

impl State {
	#[must_use]
	pub fn new() -> Self {
		let mut notes_state = tui::widgets::ListState::default();
		notes_state.select(Some(0));
		Self {
			notes_state,
			input_mode: crate::helpers::InputMode::Null,
			input: String::new(),
		}
	}

	#[must_use]
	pub fn take_input(&mut self) -> String {
		self.input.drain(..).collect()
	}

	#[inline]
	pub fn write_input(&mut self, ch: char) {
		self.input.push(ch);
	}

	#[inline]
	pub fn pop_input(&mut self) {
		self.input.pop();
	}

	pub fn select_previous_note(&mut self) -> Result<()> {
		if let Some(selected_index) = self.notes_state.selected() {
			if selected_index > 0 {
				self.notes_state.select(Some(selected_index - 1));
			} else {
				let count = crate::note::Note::get_count()
					.context("Failed to get notes count.")?;
				self.notes_state.select(Some(count - 1));
			}
		}
		Ok(())
	}

	pub fn select_next_note(&mut self) -> Result<()> {
		if let Some(selected_index) = self.notes_state.selected() {
			let notes_count = crate::note::Note::get_count()?;
			if selected_index == notes_count - 1 {
				self.notes_state.select(Some(0));
			} else {
				self.notes_state.select(Some(selected_index + 1));
			}
		}

		Ok(())
	}
}
