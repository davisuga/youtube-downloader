use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use futures_util::StreamExt;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};
use tokio::sync::mpsc;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

async fn download_youtube(url: String, output_path: PathBuf, tx: mpsc::Sender<String>) {
    info!("downloading");
    let binary = std::env::var("YTDLP_BINARY").unwrap_or("yt-dlp".to_string());
    let mut child = Command::new(binary)
        .arg(url)
        .arg("-o")
        .arg(output_path.join("%(title)s.%(ext)s").to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ytdlp process");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                tx_clone.send(line).await.expect("Failed to send stdout");
            }
        }
    });

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                tx_clone.send(line).await.expect("Failed to send stderr");
            }
        }
    });

}

#[component]
fn App() -> Element {
    let mut youtube_link = use_signal(|| "".to_string());
    let mut message = use_signal(|| "".to_string());
    let mut is_loading = use_signal(|| false);
    let mut output = use_signal(|| "".to_string());
    let mut output_path = use_signal(|| PathBuf::from("./"));

    let  download_task = use_coroutine(move |mut rx: UnboundedReceiver<()>| {
        async move {
            let (tx, mut rx_output) = mpsc::channel::<String>(100);

            rx.next().await;

            download_youtube(youtube_link(), output_path(), tx).await;

            while let Some(line) = rx_output.recv().await {
                output.with_mut(|s| {
                    s.push_str(&line);
                    s.push('\n');
                });
            }

            is_loading.set(false);
        }
    });

    rsx! {
        script { src: "https://cdn.tailwindcss.com" }
        div {
            class: "flex flex-col items-center justify-center min-h-screen bg-gray-100 p-4",
            div {
                class: "w-full max-w-3xl space-y-4",
                input {
                    class: "w-full p-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500",
                    value: "{youtube_link}",
                    placeholder: "Enter YouTube Link",
                    oninput: move |event| youtube_link.set(event.value())
                }
                div {
                    class: "flex items-center space-x-2",
                    input {
                        r#type: "file",
                        directory: true,
                        class: "hidden",
                        id: "folder-picker",
                        onchange: move |evt| {
                            if let Some(file_engine) = evt.files() {
                                let files = file_engine.files();
                                if let Some(directory) = files.first() {
                                    output_path.set(PathBuf::from(directory));
                                }
                            }
                        }
                    }
                    label {
                        r#for: "folder-picker",
                        class: "w-full bg-gray-500 text-white p-3 rounded-lg hover:bg-gray-600 transition-colors duration-300 cursor-pointer text-center !ml-0",
                        "Select Output Folder"
                    }
                }
                p {
                    class: "mt-2 text-sm text-gray-600",
                    "Current output path: {output_path().display()}"
                }
                button {
                    class: "w-full bg-blue-500 text-white p-3 rounded-lg hover:bg-blue-600 transition-colors duration-300 disabled:bg-blue-300",
                    disabled: "{is_loading}",
                    onclick: move |_| {
                        if youtube_link().is_empty() {
                            message.set("Please enter a YouTube URL.".to_string());
                            return;
                        }
                        is_loading.set(true);
                        message.set("Download started...".to_string());
                        output.set("".to_string());
                        download_task.send(());
                    },
                    if is_loading() { "Downloading..." } else { "Download" }
                }
                if !output().is_empty() {
                    div {
                        class: "mt-4 p-4 bg-black text-sm text-green-400 rounded-lg overflow-auto max-h-64",
                        pre {
                            code {
                                "{output}"
                            }
                        }
                    }
                }
            }
        }
    }
}
