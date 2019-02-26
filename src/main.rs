#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use shellfn::shell;
use std::env;
use std::error::Error;

static ICON_DIR: &str = "NOTIFY_ICONS";

#[shell]
fn notify_send(
    title: &str,
    body: &str,
    icon: &str,
    expire: &str,
) -> Result<impl Iterator<Item = String>, Box<Error>> {
    r#"
    notify-send $TITLE $BODY -i $ICON -t $EXPIRE
"#
}

#[get("/notify?<title>&<body>&<icon>&<expire>")]
fn notify(title: String, body: String, mut icon: String, mut expire: String) -> &'static str {
    let icon_path = match env::var(ICON_DIR) {
        Ok(val) => val,
        Err(_) => "$HOME/Documents/notify/icons".to_string(),
    };

    if icon == "" {
        icon = format!("{}/default.png", icon_path).to_string();
    }
    if let Ok(e) = expire.parse::<u64>() {
        if e <= 0 {
            expire = "5000".to_string();
        }
    }
    match notify_send(&title, &body, &icon, &expire) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    };
    "OK"
}

fn main() {
    rocket::ignite().mount("/", routes![notify]).launch();
}
