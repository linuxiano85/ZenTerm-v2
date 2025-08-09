use std::fmt;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EventKey(Arc<Vec<String>>);

impl EventKey {
    pub fn new<S: Into<String>>(segments: impl IntoIterator<Item = S>) -> Self {
        let v = segments.into_iter().map(Into::into).collect();
        Self(Arc::new(v))
    }

    pub fn parse(key: &str) -> Self {
        Self::new(key.split('.'))
    }

    pub fn segments(&self) -> &[String] {
        &self.0
    }
}

impl fmt::Display for EventKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.join("."))
    }
}

impl fmt::Debug for EventKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EventKey").field(&self.to_string()).finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventPattern(Vec<String>);

impl EventPattern {
    pub fn parse(pat: &str) -> Self {
        Self(pat.split('.').map(|s| s.to_string()).collect())
    }

    pub fn matches(&self, key: &EventKey) -> bool {
        pattern_matches(&self.0, key.segments())
    }

    pub fn first_segment(&self) -> Option<&str> {
        self.0.get(0).map(|s| s.as_str())
    }
}

fn pattern_matches(pattern: &[String], key: &[String]) -> bool {
    // Support:
    // *  => match exactly one segment
    // ** => match zero or more (must be last token)
    let mut i = 0usize;
    let mut j = 0usize;

    while i < pattern.len() && j < key.len() {
        let p = &pattern[i];
        match p.as_str() {
            "**" => {
                // "**" must be terminal
                return i == pattern.len() - 1;
            }
            "*" => {
                i += 1;
                j += 1;
            }
            literal if literal == key[j] => {
                i += 1;
                j += 1;
            }
            _ => return false,
        }
    }

    // Handle trailing "**"
    if i == pattern.len() - 1 && pattern[i] == "**" {
        return true;
    }

    i == pattern.len() && j == key.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        let pat = EventPattern::parse("voice.command.*");
        assert!(pat.matches(&EventKey::parse("voice.command.run")));
        assert!(!pat.matches(&EventKey::parse("voice.command.run.extra")));
        let pat2 = EventPattern::parse("plugin.**");
        assert!(pat2.matches(&EventKey::parse("plugin")));
        assert!(pat2.matches(&EventKey::parse("plugin.git")));
        assert!(pat2.matches(&EventKey::parse("plugin.git.events.push")));
        let exact = EventPattern::parse("core.start");
        assert!(exact.matches(&EventKey::parse("core.start")));
        assert!(!exact.matches(&EventKey::parse("core.stop")));
    }
}