use askama::Template;



#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;


#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub store_names: Vec<String>,
}