# wl-copyfile

Simple binary to copy a file to the clipboard from the command line, given a path.

## Usage
`wl-copyfile <path to file>`. The file will be copied to your clipboard.

## Why would I need this?

You can't simply copy a file from the terminal using commands. Of course you can copy the raw bytes (which is mostly unsupported) or use `wl-copy` with `text/uri-list` (like in [this post](https://axlefublr.github.io/uri-list/) from Axlefublr).
The problem is that, although `text/uri-list` is a commonly supported MIME type for pasting files, it's no more than a URI to where the file is, hence sandboxed apps, such as Flatpaks, won't be able to access the file and will give you a "file not found" error.

For this reason I developed this small tool, that uses the [File Transfer](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html) XDG Desktop Portal to create a proper transfer so Flatpak apps can also access the copied file. For compability, the following MIME types are copied to clipboard, mimicking GNOME's Nautilus:

- x-special/gnome-copied-files (`copy\nfile://<path to file>`);
- text/plain;charset=utf-8 (the path to the file);
- text/uri-list (`file://<path to file>`);
- application/vnd.portal.filetransfer (the FileTransfer portal token);
- application/vnd.portal.files (same as above).

The tool quietly forks itself (becoming an orphan process) in order to keep 'serving' the file without hanging the shell, and should quietly exit once something else is copied or the clipboard is cleaned.


