use cot::db::{model, Auto, LimitedString};

#[derive(Debug)]
#[model]
pub struct {{ model_name }} {
    #[model(primary_key)]
    pub id: Auto<i32>,
    pub name: LimitedString<255>,
}
