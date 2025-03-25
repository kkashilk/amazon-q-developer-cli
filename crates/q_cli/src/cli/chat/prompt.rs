use std::borrow::Cow;

use crossterm::style::Stylize;
use eyre::Result;
use rustyline::completion::{
    Completer,
    extract_word,
    FilenameCompleter,
};
use rustyline::error::ReadlineError;
use rustyline::highlight::{
    CmdKind,
    Highlighter,
};
use rustyline::history::DefaultHistory;
use rustyline::{
    Completer,
    CompletionType,
    Config,
    Context,
    EditMode,
    Editor,
    Helper,
    Hinter,
    Validator,
};
use winnow::stream::AsChar;

const COMMANDS: &[&str] = &[
    "/clear",
    "/help",
    "/acceptall",
    "/quit",
    "/profile",
    "/profile help",
    "/profile list",
    "/profile add",
    "/profile create",
    "/profile delete",
    "/profile rename",
    "/profile set",
    "/context help",
    "/context show",
    "/context show --expand",
    "/context add",
    "/context add --global",
    "/context rm",
    "/context rm --global",
    "/context clear",
    "/context clear --global",
];

pub fn generate_prompt(current_profile: Option<&str>) -> String {
    if let Some(profile_name) = &current_profile {
        if *profile_name != "default" {
            // Format with profile name for non-default profiles
            return format!("[{}] > ", profile_name);
        }
    }

    // Default prompt
    "> ".to_string()
}

/// Categorized path context patterns for better organization and maintainability
pub struct PathContextPatterns {
    /// File operation commands (cat, ls, etc.)
    pub file_commands: &'static [&'static str],
    /// Words indicating file operations (open, read, etc.)
    pub file_operations: &'static [&'static str],
    /// Path-related terms (file, path, directory, etc.)
    pub path_terms: &'static [&'static str],
    /// Special commands that work with files
    pub special_commands: &'static [&'static str],
}

impl Default for PathContextPatterns {
    fn default() -> Self {
        Self {
            file_commands: &["cat ", "ls ", "cd ", "vim ", "nano ", "less ", "more ", "grep "],
            file_operations: &["open ", "read ", "write ", "edit ", "create ", "delete ", "remove "],
            path_terms: &["file ", "path ", "directory ", "folder ", "location "],
            special_commands: &["/context add", "/context rm"],
        }
    }
}

/// A wrapper around FilenameCompleter that provides enhanced path detection
/// and completion capabilities for the chat interface.
pub struct PathCompleter {
    /// The underlying filename completer from rustyline
    filename_completer: FilenameCompleter,
    path_patterns: PathContextPatterns,
}

impl PathCompleter {
    /// Creates a new PathCompleter instance
    pub fn new() -> Self {
        Self {
            filename_completer: FilenameCompleter::new(),
            path_patterns: PathContextPatterns::default(),
        }
    }

    /// Check if the input might be referring to a file path based on context clues
    fn is_path_context(&self, line: &str) -> bool {
        // Check for file commands
        let has_file_command = self.path_patterns.file_commands
            .iter()
            .any(|&cmd| line.contains(cmd));

        // Check for file operations
        let has_file_operation = self.path_patterns.file_operations
            .iter()
            .any(|&op| line.contains(op));

        // Check for path terms
        let has_path_term = self.path_patterns.path_terms
            .iter()
            .any(|&term| line.contains(term));

        // Check for special commands
        let has_special_command = self.path_patterns.special_commands
            .iter()
            .any(|&cmd| line.contains(cmd));

        has_file_command || has_file_operation || has_path_term || has_special_command
    }

    /// Attempts to complete a file path at the given position in the line
    pub fn complete_path(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<String>), ReadlineError> {
        // Use the filename completer to get path completions
        match self.filename_completer.complete(line, pos, ctx) {
            Ok((pos, completions)) => {
                // Convert the filename completer's pairs to strings
                let file_completions: Vec<String> = completions
                    .iter()
                    .map(|pair| pair.replacement.clone())
                    .collect();

                // Return the completions if we have any
                Ok((pos, file_completions))
            }
            Err(err) => Err(err),
        }
    }

    /// Checks if the word appears to be a file path based on its syntax
    pub fn is_path_syntax(&self, word: &str) -> bool {
        word.contains('/') || word.starts_with('~') || word.starts_with('.')
    }
}

pub struct ChatCompleter {
    path_completer: PathCompleter,

}

impl ChatCompleter {
    fn new() -> Self {
        Self {
            path_completer: PathCompleter::new(),
        }
    }

    /// Complete commands that start with a slash
    fn complete_command(&self, word: &str, start: usize) -> (usize, Vec<String>) {
        (
            start,
            COMMANDS
                .iter()
                .filter(|p| p.starts_with(word))
                .map(|s| (*s).to_owned())
                .collect()
        )
    }
}

impl Completer for ChatCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let (start, word) = extract_word(line, pos, None, |c| c.is_space());

        // Handle command completion
        if word.starts_with('/') {
            return Ok(self.complete_command(word, start));
        }

        // Handle file path completion if the word contains path separators or context suggests file paths
        if self.path_completer.is_path_syntax(word) || self.path_completer.is_path_context(line) {
            if let Ok((pos, completions)) = self.path_completer.complete_path(line, pos, _ctx) {
                if !completions.is_empty() {
                    return Ok((pos, completions));
                }
            }
        }

        // Default: no completions
        Ok((start, Vec::new()))
    }
}

#[derive(Helper, Completer, Hinter, Validator)]
pub struct ChatHelper {
    #[rustyline(Completer)]
    completer: ChatCompleter,
    #[rustyline(Validator)]
    validator: (),
    #[rustyline(Hinter)]
    hinter: (),
}

impl Highlighter for ChatHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, _default: bool) -> Cow<'b, str> {
        // Check if the prompt contains a context profile indicator
        if let Some(profile_end) = prompt.find("] ") {
            // Split the prompt into context part and the rest
            let context_part = &prompt[..=profile_end];
            let rest = &prompt[(profile_end + 1)..];

            // Color the context part cyan and the rest magenta
            Cow::Owned(format!("{}{}", context_part.cyan(), rest.magenta()))
        } else {
            // Default prompt with magenta color
            Cow::Owned(prompt.magenta().to_string())
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[1m{hint}\x1b[m"))
    }

    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _kind: CmdKind) -> bool {
        false
    }
}

pub fn rl() -> Result<Editor<ChatHelper, DefaultHistory>> {
    let edit_mode = match fig_settings::settings::get_string_opt("chat.editMode").as_deref() {
        Some("vi" | "vim") => EditMode::Vi,
        _ => EditMode::Emacs,
    };
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(edit_mode)
        .build();
    let h = ChatHelper {
        completer: ChatCompleter::new(),
        hinter: (),
        validator: (),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    Ok(rl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_prompt() {
        // Test default prompt (no profile)
        assert_eq!(generate_prompt(None), "> ");
        // Test default profile (should be same as no profile)
        assert_eq!(generate_prompt(Some("default")), "> ");
        // Test custom profile
        assert_eq!(generate_prompt(Some("test-profile")), "[test-profile] > ");
        // Test another custom profile
        assert_eq!(generate_prompt(Some("dev")), "[dev] > ");
    }

    #[test]
    fn test_chat_completer_command_completion() {
        let completer = ChatCompleter::new();
        let line = "/h";
        let pos = 2; // Position at the end of "/h"

        // Create a mock context with empty history
        let empty_history = DefaultHistory::new();
        let ctx = Context::new(&empty_history);

        // Get completions
        let (start, completions) = completer.complete(line, pos, &ctx).unwrap();

        // Verify start position
        assert_eq!(start, 0);

        // Verify completions contain expected commands
        assert!(completions.contains(&"/help".to_string()));
    }

    #[test]
    fn test_chat_completer_no_completion() {
        let completer = ChatCompleter::new();
        let line = "Hello, how are you?";
        let pos = line.len();

        // Create a mock context with empty history
        let empty_history = DefaultHistory::new();
        let ctx = Context::new(&empty_history);

        // Get completions
        let (_, completions) = completer.complete(line, pos, &ctx).unwrap();

        // Verify no completions are returned for regular text
        assert!(completions.is_empty());
    }

    #[test]
    fn test_is_path_context() {
        let completer = PathCompleter::new();

        // Test positive cases
        assert!(completer.is_path_context("Please read file test.txt"));
        assert!(completer.is_path_context("Show me the path to config"));
        assert!(completer.is_path_context("List the directory contents"));
        assert!(completer.is_path_context("/context add ./myfile.txt"));
        assert!(completer.is_path_context("/context rm ./myfile.txt"));
        assert!(completer.is_path_context("Can you open this file?"));
        assert!(completer.is_path_context("Please read this for me"));
        assert!(completer.is_path_context("cat my_file.txt"));
        assert!(completer.is_path_context("ls -la"));
        assert!(completer.is_path_context("cd /usr/local"));

        // Test negative cases
        assert!(!completer.is_path_context("Hello world"));
        assert!(!completer.is_path_context("What is the weather today?"));
        assert!(!completer.is_path_context("How do I upload data using s3 cli"));
    }
}
