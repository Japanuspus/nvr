use std::{fs, path::{Path, PathBuf}};

use serde_json::Value;
use web_view::{self, Content, WebView};
use anyhow::{Result, Context};

struct Note {
    name: String,
    content: String,
}

fn get_notes(folder: &Path) -> Result<Vec<Note>> {
    let mut res = Vec::new();
    for entry in folder.read_dir()? {
        let file = entry?;
        if file.file_type().and_then(|ft| Ok(ft.is_file()))? {
            let name = file.file_name().to_string_lossy().to_string();
            if !(name.ends_with(".txt") | name.ends_with(".md")) {
                println!("Skipping {}", name);
                continue
            }
            let content = fs::read_to_string(file.path()).with_context(|| format!("Reading file: {}", name))?;
            res.push(Note{name, content});
        }
    };
    Ok(res)
}

struct NoteProvider {
    note_folder: PathBuf
}

impl NoteProvider {
    fn get_notes(&self) -> Result<Vec<Note>> {
        get_notes(&self.note_folder)
    }
}

fn handle(wv: &mut WebView<NoteProvider>, arg: &str) -> Result<(), web_view::Error> {
    let v: Value = serde_json::from_str(arg).map_err(|err| web_view::Error::Custom(Box::new(err)))?;
    if let Value::String(t) = &v["type"] {
        match &t[..] {
            "update" => {
                let np = wv.user_data();
                let notes = np.get_notes().map_err(|err| web_view::Error::Custom(Box::new(err)))?;
                let notes_value = Value::Array(notes.into_iter().map(|s| Value::String(s.name)).collect());
                wv.eval(&format!(r#"update_note_list({})"#, notes_value.to_string()))
            },
            _ => {println!("Unrecognized command: {}", t); Ok(())}
        }
    } else {
        {println!("Input is json with no 'type': {:?}", v); Ok(())}
    }
}

const HTML_CONTENT: &str = include_str!("../index.html");

fn main() -> Result<()> {
    let note_folder = PathBuf::from("./notes");
    let note_provider = NoteProvider{note_folder};

    for n in note_provider.get_notes()?.iter() {
        println!("Found note: {}", n.name);
    }

    web_view::builder()
        .title("NeVeR")
        .content(Content::Html(HTML_CONTENT))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(note_provider)
        .invoke_handler(handle)  
        .run()?;
    Ok(())
}