//! SigmaOS-zsh: Zsh shell configuration model.
//!
//! Models zsh shell themes, aliases, functions, environment variables,
//! history options, and completion settings for SigmaOS.

use std::collections::BTreeMap;
use std::fmt;

/// Zsh prompt theme configuration
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

/// Shell alias definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellAlias {
    pub name: String,
    pub command: String,
    pub description: String,
}

/// Shell function definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFunction {
    pub name: String,
    pub body: String,
    pub description: String,
}

/// Environment variable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentVar {
    pub name: String,
    pub value: String,
    pub export: bool,
}

/// Errors for zsh configuration domain
#[derive(Debug, PartialEq)]
pub enum ZshError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for ZshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for ZshError {}

/// Zsh configuration manager
pub struct ZshConfig {
    pub name: String,
    theme: ShellPromptTheme,
    aliases: BTreeMap<String, ShellAlias>,
    functions: BTreeMap<String, ShellFunction>,
    env_vars: BTreeMap<String, EnvironmentVar>,
    history_file: Option<String>,
    history_size: Option<usize>,
    autocomplete: bool,
}

impl ZshConfig {
    /// Create a new zsh configuration
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            theme: ShellPromptTheme::sovereign(),
            aliases: BTreeMap::new(),
            functions: BTreeMap::new(),
            env_vars: BTreeMap::new(),
            history_file: None,
            history_size: Some(10000),
            autocomplete: true,
        }
    }

    /// Set the prompt theme
    pub fn set_theme(&mut self, theme: ShellPromptTheme) {
        self.theme = theme;
    }

    /// Get the current theme
    pub fn theme(&self) -> &ShellPromptTheme {
        &self.theme
    }

    /// Add an alias with validation
    pub fn add_alias(&mut self, alias: ShellAlias) -> Result<(), ZshError> {
        if alias.name.is_empty() {
            return Err(ZshError::EmptyName);
        }
        if self.aliases.contains_key(&alias.name) {
            return Err(ZshError::Duplicate(alias.name.clone()));
        }
        self.aliases.insert(alias.name.clone(), alias);
        Ok(())
    }

    /// Get an alias by name
    pub fn get_alias(&self, name: &str) -> Option<&ShellAlias> {
        self.aliases.get(name)
    }

    /// Count of aliases
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }

    /// Add a function with validation
    pub fn add_function(&mut self, func: ShellFunction) -> Result<(), ZshError> {
        if func.name.is_empty() {
            return Err(ZshError::EmptyName);
        }
        if self.functions.contains_key(&func.name) {
            return Err(ZshError::Duplicate(func.name.clone()));
        }
        self.functions.insert(func.name.clone(), func);
        Ok(())
    }

    /// Get a function by name
    pub fn get_function(&self, name: &str) -> Option<&ShellFunction> {
        self.functions.get(name)
    }

    /// Count of functions
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Set an environment variable
    pub fn set_env_var(&mut self, var: EnvironmentVar) -> Result<(), ZshError> {
        if var.name.is_empty() {
            return Err(ZshError::EmptyName);
        }
        self.env_vars.insert(var.name.clone(), var);
        Ok(())
    }

    /// Get an environment variable
    pub fn get_env_var(&self, name: &str) -> Option<&EnvironmentVar> {
        self.env_vars.get(name)
    }

    /// Count of environment variables
    pub fn env_var_count(&self) -> usize {
        self.env_vars.len()
    }

    /// Set history configuration
    pub fn set_history(&mut self, file: Option<String>, size: Option<usize>) {
        self.history_file = file;
        self.history_size = size;
    }

    /// Enable or disable autocomplete
    pub fn set_autocomplete(&mut self, enabled: bool) {
        self.autocomplete = enabled;
    }

    /// Generate zsh configuration script
    pub fn generate_config(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# SigmaOS zsh configuration: {}\n", self.name));
        out.push_str("\n# Prompt theme\n");
        out.push_str(&format!("PROMPT=\"{}\"\n", self.theme.prompt_symbol));
        if self.theme.show_git_branch {
            out.push_str("# Git branch in prompt enabled\n");
        }
        out.push_str(&format!("# Color scheme: {}\n", self.theme.color_scheme));
        out.push_str("\n# Environment variables\n");
        for var in self.env_vars.values() {
            if var.export {
                out.push_str(&format!("export {}=\"{}\"\n", var.name, var.value));
            } else {
                out.push_str(&format!("{}=\"{}\"\n", var.name, var.value));
            }
        }
        out.push_str("\n# Aliases\n");
        for alias in self.aliases.values() {
            out.push_str(&format!("alias {}='{}'  # {}\n", alias.name, alias.command, alias.description));
        }
        out.push_str("\n# Functions\n");
        for func in self.functions.values() {
            out.push_str(&format!("{}() {{\n    {}\n}}  # {}\n", func.name, func.body, func.description));
        }
        if let Some(file) = &self.history_file {
            out.push_str(&format!("\n# History\nHISTFILE=\"{}\"\n", file));
        }
        if let Some(size) = self.history_size {
            out.push_str(&format!("HISTSIZE={}\nSAVEHIST={}\n", size, size));
        }
        out.push_str(&format!("\n# Autocomplete: {}\n", if self.autocomplete { "enabled" } else { "disabled" }));
        if self.autocomplete {
            out.push_str("autoload -Uz compinit && compinit\n");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_prompt_theme_aaa() {
        let theme = ShellPromptTheme::sovereign();
        assert_eq!(theme.prompt_symbol, "Σ>");
        assert!(theme.show_git_branch);
    }

    #[test]
    fn test_create_config() {
        let config = ZshConfig::new("sigmaos");
        assert_eq!(config.name, "sigmaos");
        assert_eq!(config.theme().prompt_symbol, "Σ>");
        assert!(config.autocomplete);
    }

    #[test]
    fn test_add_alias() {
        let mut config = ZshConfig::new("test");
        let alias = ShellAlias {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
            description: "long listing".to_string(),
        };
        assert!(config.add_alias(alias).is_ok());
        assert_eq!(config.alias_count(), 1);
        assert_eq!(config.get_alias("ll").unwrap().command, "ls -la");
    }

    #[test]
    fn test_empty_alias_name_rejected() {
        let mut config = ZshConfig::new("test");
        let alias = ShellAlias {
            name: "".to_string(),
            command: "ls".to_string(),
            description: "test".to_string(),
        };
        assert!(config.add_alias(alias).is_err());
    }

    #[test]
    fn test_duplicate_alias_rejected() {
        let mut config = ZshConfig::new("test");
        let alias = ShellAlias {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
            description: "first".to_string(),
        };
        config.add_alias(alias.clone()).unwrap();
        let alias2 = ShellAlias {
            name: "ll".to_string(),
            command: "ls -l".to_string(),
            description: "second".to_string(),
        };
        assert!(config.add_alias(alias2).is_err());
    }

    #[test]
    fn test_add_function() {
        let mut config = ZshConfig::new("test");
        let func = ShellFunction {
            name: "mkcd".to_string(),
            body: "mkdir -p $1 && cd $1".to_string(),
            description: "mkdir and cd".to_string(),
        };
        assert!(config.add_function(func).is_ok());
        assert_eq!(config.function_count(), 1);
    }

    #[test]
    fn test_env_var() {
        let mut config = ZshConfig::new("test");
        let var = EnvironmentVar {
            name: "EDITOR".to_string(),
            value: "vim".to_string(),
            export: true,
        };
        assert!(config.set_env_var(var).is_ok());
        assert_eq!(config.get_env_var("EDITOR").unwrap().value, "vim");
    }

    #[test]
    fn test_history_config() {
        let mut config = ZshConfig::new("test");
        config.set_history(Some("/home/user/.zsh_history".to_string()), Some(5000));
        let generated = config.generate_config();
        assert!(generated.contains("HISTFILE"));
        assert!(generated.contains("HISTSIZE=5000"));
    }

    #[test]
    fn test_generate_config() {
        let mut config = ZshConfig::new("sigmaos");
        let alias = ShellAlias {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
            description: "long listing".to_string(),
        };
        config.add_alias(alias).unwrap();
        let var = EnvironmentVar {
            name: "EDITOR".to_string(),
            value: "vim".to_string(),
            export: true,
        };
        config.set_env_var(var).unwrap();
        
        let generated = config.generate_config();
        assert!(generated.contains("PROMPT"));
        assert!(generated.contains("alias ll='ls -la'"));
        assert!(generated.contains("export EDITOR=\"vim\""));
    }

    #[test]
    fn test_set_theme() {
        let mut config = ZshConfig::new("test");
        let theme = ShellPromptTheme {
            prompt_symbol: "$".to_string(),
            show_git_branch: false,
            color_scheme: "monokai".to_string(),
        };
        config.set_theme(theme);
        assert_eq!(config.theme().prompt_symbol, "$");
        assert!(!config.theme().show_git_branch);
    }
}
