#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::Path;
use std::path::PathBuf;
use rocket::http::RawStr;

use rocket::response::{Flash, Redirect};
use rocket::request::FlashMessage;
use rocket::Request;

use std::collections::HashMap;

use rocket_contrib::templates::{Template, handlebars};

use std::{fs, io};

#[derive(serde::Serialize)]
struct TemplateContext {
    title: String,
    name: Option<String>,
    items: Vec<String>,
    location: String,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[get("/")]
fn index() -> Template {
  Template::render("about", &TemplateContext {
      title: format!("About"),
      name: None,
      items: vec![format!("Four"), format!("Five"), format!("Six")],
      location: format!(""),
      parent: "layout",
  })
}

/*
#[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
}*//*
#[get("/")]
fn index() -> &'static str { // Template {
  //let context = 
  //Template::render("index", &context)
  
  //"Hello, world!"
}*/

#[get("/r")]
fn redirect() -> Flash<Redirect> {
  // Redirect::to(uri!(index))
  Flash::success(Redirect::to(uri!(flash)), "Successfully logged out.")
}

#[get("/flash")]
fn flash(flash: FlashMessage) -> String {
  
  //  "Hello, world!"
  let msg = flash.msg();
  
  msg.to_string()
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> Template {//-> String {
  let mut map = std::collections::HashMap::new();
  map.insert("name", name.as_str());
  Template::render("hello", &map)
}

#[get("/games")]
fn game_grid() -> Template {
  let mut entries = fs::read_dir("./games/").unwrap()
        .map(|res| res.map(|e| e.path().to_str().unwrap().to_string().split_off(8)))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
  println!("{:?}", entries);
  
  Template::render("game_grid", &TemplateContext {
    title: format!("Games"),
    name: None,
    items: entries,
    location: format!(""),
    parent: "layout",
  })
}

#[get("/<game_name>")]
fn game(game_name: &RawStr) -> Template {
  Template::render("game", &TemplateContext {
    title: format!("{}", game_name),
    name: None,
    items: vec![format!("")],
    location: format!("{}", game_name),
    parent: "layout",
  })
}

#[get("/resume")]
fn resume() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/resume.pdf")).ok()
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/css/<file..>")]
fn css(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/css/").join(file)).ok()
}

#[get("/games/images/<file..>")]
fn game_images(file: PathBuf) -> Option<NamedFile> {
  let p = Path::new("games/").join(file);
  println!("stuff is here?: {:?}", p);
  NamedFile::open(p).ok()
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
  let mut map = std::collections::HashMap::new();
  map.insert("path", req.uri().path());
  Template::render("error/404", &map)
}

use self::handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};

fn wow_helper(
    h: &Helper<'_, '_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_>,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn main() {
    rocket::ignite()
      .mount("/", routes![index, redirect, flash, hello, game_grid, game, game_images, resume, files, css])
      .register(catchers![not_found])
      .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
      .launch();
}
