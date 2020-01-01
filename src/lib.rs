use serde::{Deserialize, Serialize};
use crate::AuthEvent::{Created, Login};

// TODO move to a module

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
    Empty
}

impl mod_event::PublicEvent for AuthEvent {
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
                AuthEvent::Empty
            }
        }
    }

    fn stream_name(&self) -> &'static str {
        return STREAM_NAME;
    }

    fn get_json(&self) -> Result<(&'static str, String), &str> {
        match self {
            Created(account_created) => {
                Ok((ACCOUNT_CREATED, serde_json::to_string(account_created).unwrap()))
            }
            Login(logged_in) => {
                Ok((LOGGED_IN, serde_json::to_string(logged_in).unwrap()))
            }
            _ => {
                Err("cannot get json from Empty")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mod_event::PublicEvent;

    #[test]
    fn it_works() {
        let data = crate::AuthEvent::Created(
            crate::AccountCreated {
                uuid: "b7b9749f-6baf-43fa-be79-15ead7cafcca".to_string(),
                name: "Aedius".to_string(),
            });

        let (event_type, event_json) = data.get_json().unwrap();

        assert_eq!(
            event_json,
            r#"{"uuid":"b7b9749f-6baf-43fa-be79-15ead7cafcca","name":"Aedius"}"#);
    }
}