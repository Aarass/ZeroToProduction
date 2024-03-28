use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail {
    email: String,
}
impl SubscriberEmail {
    pub fn parse(email: String) -> Result<Self, String> {
        if validate_email(&email) == false {
            return Err(String::from("Couldn't parse email"));
        }
        Ok(SubscriberEmail { email: email })
    }
}
impl TryFrom<String> for SubscriberEmail {
    type Error = String;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::parse(text)
    }
}
impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.email
    }
}
#[cfg(test)]
mod tests {
    use core::panic;

    use crate::domain::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();

        if let Ok(_) = SubscriberEmail::parse(email) {
            panic!();
        }
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();

        if let Ok(_) = SubscriberEmail::parse(email) {
            panic!();
        }
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();

        if let Ok(_) = SubscriberEmail::parse(email) {
            panic!();
        }
    }
}
