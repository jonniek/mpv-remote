#[macro_use] extern crate rocket;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::State;

use std::env;
use std::{fs::File, io::Write};

struct AppState {
  ipc_socket_path: String
}

enum Command {
  TogglePause,
  Seek(i32),
}

const SEEK_BACK: Command = Command::Seek(-5);
const SEEK_FORWARD: Command = Command::Seek(5);
const PAUSE: Command = Command::TogglePause;

fn command_string(command: Command) -> String {
  match command {
    Command::TogglePause => {
      "{\"command\":[\"cycle\",\"pause\"]}\n".to_string()
    },
    Command::Seek(amount) => {
      format!("{{\"command\":[\"seek\",{},\"exact\"]}}\n", amount)
    }
  }
}

fn write_file(bytes: &[u8], path: &str) -> std::io::Result<()> {
  let mut file = File::create(path)?;
  file.write_all(bytes)?;
  Ok(())
}

fn write_command(com: Command, path: &str) {
  let command = command_string(com);

  match write_file(command.as_bytes(), path) {
    Ok(_) => (),
    Err(e) => eprintln!("{}", e),
  }
}

fn get_socket_path() -> String {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    panic!("IPC socket path is required argument")
  }
  args[1].to_owned()
}

#[post("/pause")]
fn pause(state: &State<AppState>) -> Redirect {
  write_command(PAUSE, &state.ipc_socket_path);

  Redirect::to(uri!(index))
}

#[post("/seek-back")]
fn seek_back(state: &State<AppState>) -> Redirect {
  write_command(SEEK_BACK, &state.ipc_socket_path);

  Redirect::to(uri!(index))
}

#[post("/seek-forward")]
fn seek_forward(state: &State<AppState>) -> Redirect {
  write_command(SEEK_FORWARD, &state.ipc_socket_path);

  Redirect::to(uri!(index))
}

#[get("/")]
fn index() -> Html<&'static str> {
  Html(PAGE)
}

#[launch]
fn rocket() -> _ {
  let ipc_socket_path = get_socket_path();

  let figment = rocket::Config::figment()
    .merge(("address", "0.0.0.0"))
    .merge(("port", 8000));

  rocket::custom(figment)
    .mount("/", routes![index])
    .mount("/", routes![pause])
    .mount("/", routes![seek_forward])
    .mount("/", routes![seek_back])
    .manage(AppState { ipc_socket_path })
}

const PAGE: &'static str = r#"
<title>mpv-remote</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
  html {
    padding: 16px;
    background: #111;
  }
  body {
    height: 100%;
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-auto-rows: 100px;
    gap: 16px;
  }
  button {
    font-size: 25px;
    font-weight: bold;
    color: #fff;
    background: #61931a;
    width: 100%;
    border: none;
    border-radius: 4px;
    height: 100%;
  }
</style>

<form action="/seek-back" method="post">
  <button type="submit">&lt&lt</button>
</form>

<form action="/seek-forward" method="post">
    <button type="submit">&gt&gt</button>
</form>

<form action="/pause" method="post">
    <button type="submit">pause</button>
</form>
"#;