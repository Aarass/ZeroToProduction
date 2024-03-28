use super::name::*;

#[derive(Debug)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}

impl NewSubscriber {
    pub fn parse(name: String, email: String) -> Result<NewSubscriber, String> {
        let name: SubscriberName = SubscriberName::parse(name)?;
        let email: String = email;

        Ok(NewSubscriber {
            name: name,
            email: email,
        })
    }
}
