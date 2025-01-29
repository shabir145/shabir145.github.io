# Organize Tool 🗂️

Effortlessly organize your files in directories.

## About 🛠️

The Organize Tool helps you efficiently manage and sort files in your directories. It categorizes files into folders based on their file extensions and can further organize them by the initial letter of the file name. The tool creates descriptive folders like ```pdf_files_with_a_initial```  for PDF files starting with the letter "a" and ```txt_files_with_b_initial``` for TXT files starting with the letter "b". It also automatically cleans up any empty directories after organizing your files, keeping your file structure clean and clutter-free.

## Benefits ✨

- ⏳ **Time-Saving**: Automatically sorts files by extension and initial, saving you time on manual organization.
- 📁 **Enhanced Organization**: Creates organized directories based on file types and file name initials (e.g., ```pdf_files_with_a_initial```), giving you a more structured file system.
- 🧹 **Clutter-Free**: Cleans up empty directories after organizing your files, keeping your workspace neat.
- 🖥️ **Cross-Platform**: Supports both Linux and Windows environments, making it versatile for different operating systems.
- 👍 **Ease of Use**: Simple setup and intuitive operation, allowing you to quickly organize your files with minimal effort.

## How It Works 🔧

The Organize Tool scans a directory, identifies files by their extensions and optionally by the initial letter of the file name, and moves them into corresponding subdirectories. The tool creates descriptive folders for better organization. For example:

- `.txt` files go to a folder named `txt_files_with_a_initial` (for files starting with "a") or `txt_files_with_b_initial` (for files starting with "b"), depending on the specified initials.
- `.jpg` files go to a folder named `jpg_files_with_a_initial` or `jpg_files_with_b_initial` (if initials are specified).
- Files without a specified initial are placed into folders based solely on their extensions (e.g., `pdf`, `txt`).
- If no initials are provided, the files are categorized by their extensions only (e.g., all `.pdf` files will go into a `pdf` folder).
- Any empty directories are automatically removed after the files are organized.


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

## Instructions 📚

To use the Organize Tool, follow these simple steps:

### 1. Basic Usage (Organize by Extension)
To organize files based on their extensions, simply run the tool with the directory you want to organize:

```bash
organize <directory_path>
```

Example:

```bash
organize /path/to/directory
```

This will organize files into subdirectories based on their extensions (e.g., `.txt` files will go into a `txt` folder, `.pdf` files into a `pdf` folder).

### 2. Organize by Initials
To organize files by both their extension and the initial letter of the file name, use the `-n` option followed by the initial letter you want to filter by:

```bash
organize <directory_path> -n <initial>
```

Example:

```bash
organize /path/to/directory -n a
```

This will move all files starting with "a" into their respective extension-based directories (e.g., `pdf_files_with_a_initial`, `txt_files_with_a_initial`).

### 3. Organize by Specific File Extension 
To organize only files of a specific extension, use the `--extension` option followed by the extension (without the dot):

```bash
organize <directory_path> --extension <extension>
```

Example:

```bash
organize /path/to/directory --extension pdf
```

This will only move `.pdf` files into a `pdf` folder, and ignore other file types.

### 4. Combine Initials and Extension Filters
You can combine both the `-n` (initial) and `--extension` filters to organize files based on both the initial letter and the extension:

```bash
organize <directory_path> -n <initial> --extension <extension>
```

Example:

```bash
organize /path/to/directory -n a --extension pdf
```

This will move `.pdf` files starting with "a" into a `pdf_files_with_a_initial` folder.

### 5. Automatically Clean Up Empty Directories
After organizing your files, the tool will automatically remove any empty directories left behind in the process, ensuring a clutter-free workspace.

## About the Creator 👤

Created by **Abdul Aziz Azizi**. View the project (https://shabir145.github.io/).






