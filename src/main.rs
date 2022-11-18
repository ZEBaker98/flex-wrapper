use std::env;

#[macro_use] extern crate rocket;
use tokio::process::Command;

use rocket::{State};

mod wrapped_process;
use wrapped_process::WrappedProcess;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/status")]
async fn status(wp: &State<WrappedProcess>) -> String {
    match wp.inner().process().await {
        Ok(child) => {
            match &*child {
                Some(process) => format!("Process Running with id {}", process.id().unwrap_or_default()),
                None => String::from("Process is not running"),
            }
        },
        Err(e) => format!("Unexpected error {:?}", e),
    }
}

#[get("/start")]
async fn start(wp: &State<WrappedProcess>) -> String {
    match wp.inner().start().await {
        Ok(()) => String::from("Process Started"),
        Err(e) => format!("Unexpected error {:?}", e),
    }
}

#[get("/kill")]
async fn kill(wp: &State<WrappedProcess>) -> String {
    match wp.inner().kill().await {
        Ok(()) => String::from("Process Terminated"),
        Err(e) => format!("Unexpected error {:?}", e),
    }
}

fn build_command() -> Option<Command> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut c = Command::new(&args[1]);
        if args.len() > 2 {
            c.args(&args[2..]);
        }
        Some(c)
    } else {
        None
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    let wp = WrappedProcess::new(build_command().expect("flex-wrapper expects a command to wrap: flex-wrapper <command> [command-arguments]"));

    let rocket = rocket::build()
    .manage(wp)
    .mount("/", routes![index])
    .mount("/process", routes![status, start, kill]).launch().await?;

    println!("Rocket has stopped, cleaning up");
    match rocket.state::<WrappedProcess>().unwrap().kill().await {
        Ok(_) => println!("Wrapped process has been stopped"),
        Err(e) => eprintln!("Unexpected error {:?}", e),
    }
    Ok(())
}