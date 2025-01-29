# Organize Tool 🗂️

Effortlessly organize your files in directories.

## About 🛠️

The Organize Tool helps you efficiently manage and sort files in your directories. It categorizes files into folders based on their file extensions and can further organize them by the initial letter of the file name. The tool creates descriptive folders like ```pdf_files_with_a_initial```  for PDF files starting with the letter "a" and ```txt_files_with_b_initial``` for TXT files starting with the letter "b". It also automatically cleans up any empty directories after organizing your files, keeping your file structure clean and clutter-free.

## Benefits ✨

- ⏳ **Time-Saving**: Automatically sorts files, reducing manual effort.
- 📁 **Enhanced Organization**: Creates separate directories for different file types.
- 🖥️ **Cross-Platform**: Supports both Linux and Windows environments.
- 👍 **Ease of Use**: Simple setup and intuitive operation.

## How It Works 🔧

The Organize Tool scans a directory, identifies files by their extensions, and moves them into corresponding subdirectories. For example:

- `.txt` files go to a folder named `txt`.
- `.jpg` files go to a folder named `jpg`.
- Unknown file types are placed in a folder named `unknown_file_type`.

## Download 📦

Select your platform to download:

- **Linux** 
- **Windows** 

## Setup Instructions 🛠️


### For Linux:
1. Move the tool to a globally accessible location:
    ```bash
    export PATH=$PATH:$HOME/path/to/directory
    ```
2. Ensure the directory is accessible globally.

### For Windows:
1. Add the tool to your system PATH:
    ```bash
    setx PATH "%PATH%;C:\path\to\directory" /M
    ```
2. Run CMD as administrator to apply the changes.

Note: path/to/directory for both Windows and Linux is the absolute path for the organize executable file where you extract. 
## About the Creator 👤

Created by **Abdul Aziz Azizi**. View the project (https://shabir145.github.io/).
