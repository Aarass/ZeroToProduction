use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName {
    name: String,
}

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty = name.trim().is_empty();

        let is_too_long = name.graphemes(true).count() > 256;

        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let mut contains_forbidden = false;
        for char in name.chars() {
            if forbidden_chars.contains(&char) {
                contains_forbidden = true;
            }
        }

        if !is_empty && !is_too_long && !contains_forbidden {
            Ok(SubscriberName { name: name })
        } else {
            Err(String::from("Couldn't parse name"))
        }
    }
}
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
