
use serde::{Deserialize, Serialize };


#[derive(Deserialize, Serialize)]
pub struct AccountCreated {
    pub uuid: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        let data =  crate::AccountCreated{
            uuid: "b7b9749f-6baf-43fa-be79-15ead7cafcca".to_string(),
            name: "Aedius".to_string(),
        };

        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"uuid":"b7b9749f-6baf-43fa-be79-15ead7cafcca","name":"Aedius"}"#);
    }
}