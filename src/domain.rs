use unicode_segmentation::UnicodeSegmentation;

// can't construct instance outside of domain, moreover can only construct thru parse fn
// "parse, don't validate"
pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_emptry_or_whitespace = s.trim().is_empty();
        let is_too_log = s.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden_chars =
            s.chars().any(|c| forbidden_chars.contains(&c));

        if is_emptry_or_whitespace || is_too_log || has_forbidden_chars {
            panic!("{} is not a valid subscriber name", s)
        } else {
            Self(s)
        }
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
