use crate::models::{{ model_name }}::{{ model_name }};

use cot::{App, AppBuilder, Project, static_files, Template};
use cot::db::{DatabaseError, Database};
use cot::html::Html;
use cot::request::extractors::StaticFiles;

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    static_files: StaticFiles,
}
// A function for returning a rendered webpage
pub async fn index(static_files: StaticFiles) -> cot::Result<Html> {
    let index_template = IndexTemplate { static_files };
    let rendered = index_template.render()?;

    Ok(Html::new(rendered))
}

// A function for creating a new model
pub async fn create(db: Database, model: {{ model_name }}) -> Result<(), ()>  {
    Ok(())
}

// #[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
// pub struct ListResponse {
//     status: u16,
//     data: Vec<{{ model_name }}>
// }
// An example function to return a Json of the list of responses
// pub async fn list(db: Database, model: {{ model_name }}) -> Json<ListResponse> {
//     Query::<{{ model_name }}>::new().all(&db).await.unwrap();
//     Json(ListResponse{
//         status: cot::http::StatusCode::OK.as_u16(),
//         data: Query
//     })
// }
