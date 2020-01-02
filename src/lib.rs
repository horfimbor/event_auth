use serde::{Deserialize, Serialize};

const STREAM_NAME: &'static str = "account";
pub const ACCOUNT_CREATED: &'static str = "account_created";
pub const LOGGED_IN: &'static str = "logged_in";


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
pub enum AuthEventList {
    Created(AccountCreated),
    Login(LoggedIn),
    Empty
}

pub struct GlobalAuthEvent{
    events: AuthEventList,
}

impl GlobalAuthEvent {
    pub fn from_enum(events :AuthEventList )-> Self{
        GlobalAuthEvent{
            events
        }
    }
}

impl mod_event::PublicEvent for GlobalAuthEvent {

    fn from_json(event_type: &str, json: &str) -> Self {
        match event_type {
            ACCOUNT_CREATED => {
                let created: AccountCreated = serde_json::from_str(json).unwrap();
                GlobalAuthEvent {
                    events: AuthEventList::Created(created)
                }
            }
            LOGGED_IN => {
                let login: LoggedIn = serde_json::from_str(json).unwrap();
                GlobalAuthEvent {
                    events: AuthEventList::Login(login)
                }
            }
            _ => {
                GlobalAuthEvent {
                    events: AuthEventList::Empty
                }
            }
        }
    }

    fn stream_name(&self) -> &'static str {
        return STREAM_NAME;
    }

    fn get_json(&self) -> Result<(&'static str, String), &str> {
        match &self.events {
            AuthEventList::Created(account_created) => {
                Ok((ACCOUNT_CREATED, serde_json::to_string(account_created).unwrap()))
            }
            AuthEventList::Login(logged_in) => {
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
    fn lifecycle_from_enum() {
        let logged_in = crate::LoggedIn{
            uuid : "cbae7c51-2068-4cb4-b4c4-e118d675f277".to_string()
        };
        let data = crate::AuthEventList::Login(logged_in);

        let obj = crate::GlobalAuthEvent::from_enum( data);

        assert_eq!(&obj.stream_name(), &"account");

        let (event_type, event_json) = obj.get_json().unwrap();

        assert_eq!(
            event_json,
            r#"{"uuid":"cbae7c51-2068-4cb4-b4c4-e118d675f277"}"#);

        assert_eq!(
            event_type,
            "logged_in");
    }

    #[test]
    fn lifecycle_from_json() {
        let data = r#"{"name":"Aedius","uuid":"b7b9749f-6baf-43fa-be79-15ead7cafcca","foo":"bar"}"#;

        let obj = crate::GlobalAuthEvent::from_json(crate::ACCOUNT_CREATED, data);

        assert_eq!(&obj.stream_name(), &"account");

        let (event_type, event_json) = obj.get_json().unwrap();

        assert_eq!(
            event_json,
            r#"{"uuid":"b7b9749f-6baf-43fa-be79-15ead7cafcca","name":"Aedius"}"#);

        assert_eq!(
            event_type,
            "account_created");
    }

}