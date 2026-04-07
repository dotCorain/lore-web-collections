pub enum LineTypes {
    /// empty line
    Empty,

    /// url link line
    UrlLink(String, String),

    /// lore link line
    LoreLink(String, String),

    /// comment line
    Comment(String),

    /// domain title line
    DomainTitle(String),

    /// placeholder line
    Placeholder,

    /// atom line
    Atom(String),
}