use tera::Context;
use warp::{reply, Rejection, Reply};

use crate::template_setup::tera::render;

pub async fn get() -> Result<impl Reply, Rejection> {
    let mut ctx = Context::new();

    let name = "Steadylearner";
    ctx.insert("name", &name);

    let payload = render("index.tera", &ctx)?;
    Ok(reply::html(payload))
}
