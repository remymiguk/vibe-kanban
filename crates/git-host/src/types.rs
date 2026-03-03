// Existing code above

pub enum ProviderKind {
    Github,
    Gitlab,
    Gitea, // Newly added variant
    Unknown,
}

impl std::fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProviderKind::Github => write!(f, "Github"),
            ProviderKind::Gitlab => write!(f, "Gitlab"),
            ProviderKind::Gitea => write!(f, "Gitea"), // Handle Gitea	ext{ }
            ProviderKind::Unknown => write!(f, "Unknown"),
        }
    }
}
