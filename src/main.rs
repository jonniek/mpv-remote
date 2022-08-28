#[macro_use] extern crate rocket;
use rocket::response::content::Html;
use rocket::response::Redirect;
use rocket::State;

use std::env;
use std::{io::Write};

#[cfg(windows)]
use std::fs::File;

#[cfg(unix)]
use std::os::unix::net::{UnixStream};

const VALID_LONG_COMMANDS: [&'static str;3] = [
  "add",
  "cycle",
  "set",
];

const VALID_SHORT_COMMANDS: [&'static str;1] = [
  "seek",
];

struct AppState {
  ipc_socket_path: String
}

struct Args {
  ipc_socket_path: String,
  address: String,
  port: i32,
}

#[cfg(windows)]
fn write_file(bytes: &[u8], path: &str) -> std::io::Result<()> {
  let mut file = File::create(path)?;
  file.write_all(bytes)?;

  Ok(())
}

#[cfg(unix)]
fn write_file(bytes: &[u8], path: &str) -> std::io::Result<()> {
  let mut unix_stream = UnixStream::connect(path)?;
  unix_stream.write_all(bytes)?;

  Ok(())
}

fn write_raw_command(com: &str, path: &str) {
  println!("Writing command to IPC socket: {}", com);
  match write_file(com.as_bytes(), path) {
    Ok(_) => (),
    Err(e) => eprintln!("{}", e),
  }
}

fn get_args() -> Args {
  let args: Vec<String> = env::args().collect();
  if args.len() != 4 {
    panic!("IPC socket path, address, port are required params")
  }

  Args {
    ipc_socket_path: args[1].to_owned(),
    address: args[2].to_owned(),
    port: args[3].to_owned().parse::<i32>().expect("Parsing port failed")
  }
}


#[post("/<command>/<name>/<value>")]
fn long_command(command: &str, name: &str, value: &str, state: &State<AppState>) -> Redirect {

  if VALID_LONG_COMMANDS.contains(&command) {
    let command_string = format!("{{\"command\":[\"osd-msg\",\"{}\",\"{}\",\"{}\"]}}\n", command, name, value);
    write_raw_command(&command_string, &state.ipc_socket_path);
  } else {
    eprintln!("unexpected command {}", command);
  }

  Redirect::to(uri!(index))
}

#[post("/<command>/<value>")]
fn short_command(command: &str, value: &str, state: &State<AppState>) -> Redirect {

  if VALID_SHORT_COMMANDS.contains(&command) {
    let command_string = format!("{{\"command\":[\"osd-msg\",\"{}\",\"{}\"]}}\n", command, value);
    write_raw_command(&command_string, &state.ipc_socket_path);
  } else {
    eprintln!("unexpected command {}", command);
  }

  Redirect::to(uri!(index))
}

#[get("/")]
fn index() -> Html<&'static str> {
  Html(PAGE)
}

#[launch]
fn rocket() -> _ {
  let args = get_args();

  let figment = rocket::Config::figment()
    .merge(("address", args.address))
    .merge(("port", args.port));

  rocket::custom(figment)
    .mount("/", routes![index])
    .mount("/", routes![short_command])
    .mount("/", routes![long_command])
    .manage(AppState { ipc_socket_path: args.ipc_socket_path })
}

const PAGE: &'static str = r#"
<head>
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

    .wide {
      grid-column: 1 / 3;
    }

    .narrow {
      grid-column:
    }

  </style>
</head>
<body>
  <form action="/cycle/pause/up" method="post" class="wide">
    <button type="submit">pause</button>
  </form>

  <form action="/seek/-5" method="post">
    <button type="submit">&lt&lt</button>
  </form>

  <form action="/seek/5" method="post">
      <button type="submit">&gt&gt</button>
  </form>

  <form action="/add/volume/-10" method="post">
    <button type="submit">- vol</button>
  </form>

  <form action="/add/volume/10" method="post">
    <button type="submit">+ vol</button>
  </form>

  <form action="/cycle/aid/up" method="post">
    <button type="submit">aid</button>
  </form>

  <form action="/cycle/sid/up" method="post">
    <button type="submit">sid</button>
  </form>

  <form action="/cycle/secondary-sid/up" method="post">
    <button type="submit">ssid</button>
  </form>
</body>
"#;