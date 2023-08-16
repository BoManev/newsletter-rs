use unicode_segmentation::UnicodeSegmentation;

// can't construct instance outside of domain, moreover can only construct thru parse fn
// "parse, don't validate"
#[derive(Debug)]
pub struct SubscriberName(String);

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    #[tracing::instrument(name = "Parse Subscriber")]
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_emptry_or_whitespace = s.trim().is_empty();
        tracing::info!("{is_emptry_or_whitespace} is_empty for {s}");
        let is_too_log = s.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden_chars =
            s.chars().any(|c| forbidden_chars.contains(&c));

        if is_emptry_or_whitespace || is_too_log || has_forbidden_chars {
            Err(format!("{s} is not a valid subscriber name"))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn name_longer_than_256_grapheme_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_contains_an_invalid_chars_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn valid_name_is_parsed_successfully() {
        let name = "Bo Manev".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
