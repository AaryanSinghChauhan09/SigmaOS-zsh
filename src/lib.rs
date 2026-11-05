#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPromptTheme {
    pub prompt_symbol: String,
    pub show_git_branch: bool,
    pub color_scheme: String,
}

impl ShellPromptTheme {
    pub fn sovereign() -> Self {
        Self {
            prompt_symbol: "Σ>".into(),
            show_git_branch: true,
            color_scheme: "nord".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shell_prompt_theme_aaa() {
        // Arrange & Act
        let theme = ShellPromptTheme::sovereign();
        // Assert
        assert_eq!(theme.prompt_symbol, "Σ>");
        assert!(theme.show_git_branch);
    }
}
