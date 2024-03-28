use crate::routes::UserData;

use super::{SubscriberEmail, SubscriberName};

#[derive(Debug)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl NewSubscriber {
    pub fn parse(name: String, email: String) -> Result<NewSubscriber, String> {
        let name: SubscriberName = SubscriberName::parse(name)?;
        let email: SubscriberEmail = SubscriberEmail::parse(email)?;

        Ok(NewSubscriber {
            name: name,
            email: email,
        })
    }
}
impl TryFrom<UserData> for NewSubscriber {
    type Error = String;

    fn try_from(form: UserData) -> Result<Self, Self::Error> {
        let new_sub = NewSubscriber {
            name: form.name.try_into()?,
            email: form.email.try_into()?,
        };
        Ok(new_sub)
    }
}
