/// Downloaded filter lines iterator.
pub(crate) struct FilterCursor {
    /// Current line
    pub(super) lineno: usize,
    /// Normalized download url (scheme://(url)?path)
    pub(super) normalized_url: String,
    /// Lines of file
    contents: Vec<String>,
}

impl FilterCursor {
    pub(super) fn new(normalized_url: String, contents: String) -> Self {
        Self {
            lineno: 0,
            normalized_url,
            contents: contents.split('\n').map(str::to_string).collect(),
        }
    }

    /// Gets next line of file
    pub(super) fn next_line(&mut self) -> Option<String> {
        let mut out: Option<String> = None;
        if self.contents.len() > self.lineno {
            out = Some(self.contents[self.lineno].clone());

            self.lineno += 1;
        };

        out
    }
}
