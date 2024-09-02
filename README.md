# YouTube Downloader with Dioxus

This project is a simple YouTube video downloader built using Dioxus, a framework for building user interfaces with Rust. It allows users to input a YouTube URL, select an output directory, and download the video to the specified location. The application also provides real-time feedback on the download progress.

![YouTube Downloader Screenshot](https://github.com/davisuga/youtube-downloader/assets/screenshot.png)

## Features

- **YouTube Video Download**: Enter a YouTube URL and download the video to a specified directory.
- **Real-time Feedback**: Displays real-time output from the download process.
- **Tailwind CSS**: Utilizes Tailwind CSS for styling, ensuring a modern and responsive UI.

## Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **yt-dlp**: This application uses `yt-dlp` for downloading YouTube videos. Ensure it is installed and available in your PATH. You can download it from [yt-dlp GitHub](https://github.com/yt-dlp/yt-dlp).

## Setup

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/davisuga/youtube-downloader.git
    cd youtube-downloader
    ```
2. **Run the Application**:
    ```sh
    dx serve --platform desktop
    ```

## Usage

1. **Enter YouTube URL**: Input the URL of the YouTube video you wish to download.
2. **Select Output Folder**: Click the "Select Output Folder" button to choose the directory where the video will be saved.
3. **Start Download**: Click the "Download" button to begin the download process. The button will display "Downloading..." while the download is in progress.
4. **View Output**: The application will display real-time output from the download process in the console.

## Configuration

- **Custom Binary Path**: If `yt-dlp` is not in your PATH, you can set the `YTDLP_BINARY` environment variable to the path of the `yt-dlp` binary.
    ```sh
    export YTDLP_BINARY=/path/to/yt-dlp
    ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any bugs or feature requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) for providing a great framework for building UIs with Rust.
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the powerful YouTube download utility.
- [Tailwind CSS](https://tailwindcss.com/) for the sleek and responsive styling.