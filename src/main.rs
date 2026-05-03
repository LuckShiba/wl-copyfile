use ashpd::documents::FileTransfer;
use futures_executor::block_on;
use libc::daemon;
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;
use wl_clipboard_rs::copy::{self, MimeSource, MimeType, Source};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    let file_path = &args[1];

    let path = Path::new(file_path);
    if !path.exists() || !path.is_file() {
        eprintln!(
            "Error: File '{}' does not exist or is not accessible.",
            file_path
        );
        exit(1);
    }

    let abs_path = fs::canonicalize(path)
        .unwrap()
        .to_string_lossy()
        .to_string();

    // we need to keep the process alive so it's still serving the FileTransfer portal by the end of it.
    unsafe {
        let res = daemon(0, 0);
        if res < 0 {
            panic!("Couldn't create daemon! Error: {}", res);
        }
    }

    block_on(work(abs_path)).unwrap();
}

async fn work(file: String) -> ashpd::Result<()> {
    let file_transfer = FileTransfer::new().await?;
    let key = file_transfer.start_transfer(false, true).await?;

    let fds = vec![std::fs::File::open(&file).unwrap()];

    file_transfer
        .add_files(key.as_str(), fds.as_slice())
        .await?;

    let mut options = copy::Options::new();
    options.foreground(true);

    let prepared_copy = options
        .prepare_copy_multi(vec![
            MimeSource {
                mime_type: MimeType::Specific("x-special/gnome-copied-files".into()),
                source: Source::Bytes(format!("copy\nfile://{}", file).as_bytes().into()),
            },
            MimeSource {
                mime_type: MimeType::Specific("application/vnd.portal.filetransfer".into()),
                source: Source::Bytes(key.as_bytes().into()),
            },
            MimeSource {
                mime_type: MimeType::Specific("application/vnd.portal.files".into()),
                source: Source::Bytes(key.as_bytes().into()),
            },
            MimeSource {
                mime_type: MimeType::Specific("text/uri-list".into()),
                source: Source::Bytes(format!("file://{}", file).as_bytes().into()),
            },
            MimeSource {
                mime_type: MimeType::Specific("text/plain".into()),
                source: Source::Bytes(file.as_bytes().into()),
            },
        ])
        .unwrap();

    drop(prepared_copy.serve());

    Ok(())
}
