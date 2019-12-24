use serde::{Deserialize, Serialize};
use crate::AuthEvent::{Created, Login};

// TODO move to a module
trait PublicEvent {
    fn from_json(event_type: &str, json: &str) -> Self;
    fn stream_name(&self) -> &'static str;
    fn get_json(&self) -> (&str, &str);
}

const STREAM_NAME: &'static str = "account";
const ACCOUNT_CREATED: &'static str = "account_created";
const LOGGED_IN: &'static str = "logged_in";


#[derive(Deserialize, Serialize)]
pub struct AccountCreated {
    pub uuid: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoggedIn {
    pub uuid: String,
}


#[derive(Deserialize, Serialize)]
pub enum AuthEvent {
    Created(AccountCreated),
    Login(LoggedIn),
    None
}

impl PublicEvent for AuthEvent {
    fn from_json(event_type: &str, json: &str) -> Self {
        match event_type {
            ACCOUNT_CREATED => {
                let created: AccountCreated = serde_json::from_str(json).unwrap();
                AuthEvent::Created(created)
            }
            LOGGED_IN => {
                let login: LoggedIn = serde_json::from_str(json).unwrap();
                AuthEvent::Login(login)
            }
            _ => {
                AuthEvent::None
            }
        }
    }

    fn stream_name(&self) -> &'static str {
        return STREAM_NAME;
    }

    fn get_json(&self) -> (&str, &str) {
        match self {
            Created(account_created) => {
                (ACCOUNT_CREATED, serde_json::to_string(account_created).unwrap().as_str())
            }
            Login(logged_in) => {
                (LOGGED_IN, serde_json::to_string(logged_in).unwrap().as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let data = crate::AuthEvent::AccountCreated(
            crate::AccountCreated {
                uuid: "b7b9749f-6baf-43fa-be79-15ead7cafcca".to_string(),
                name: "Aedius".to_string(),
            });

        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"uuid":"b7b9749f-6baf-43fa-be79-15ead7cafcca","name":"Aedius"}"#);
    }
}