# OOXML Manipulator

A Rust program that extracts Word files into their [OOXML](https://en.wikipedia.org/wiki/Office_Open_XML) representation and provides utilities for analyzing and editing the internal structure.

## Table of Contents

- [OOXML Manipulator](#ooxml-manipulator)
  - [Table of Contents](#table-of-contents)
  - [Summary](#summary)
  - [Features](#features)
  - [Motivation](#motivation)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Quick Start](#quick-start)
    - [Feature Guide](#feature-guide)
      - [1. Word File Extraction](#1-word-file-extraction)
      - [2. Re-zip Extracted Folder](#2-re-zip-extracted-folder)
      - [3. Summarize Structure](#3-summarize-structure)
      - [4. Analyze Custom XML](#4-analyze-custom-xml)
      - [5. Edit Custom XML](#5-edit-custom-xml)
      - [6. Watch for Changes](#6-watch-for-changes)
  - [Sample Data](#sample-data)
    - [Available Sample Files](#available-sample-files)
    - [Testing Custom XML Features](#testing-custom-xml-features)
    - [Quick Test Workflow](#quick-test-workflow)
  - [Feature Details](#feature-details)
    - [Word File Structure](#word-file-structure)
    - [User Flow](#user-flow)
  - [Project Structure](#project-structure)
  - [Known Issues](#known-issues)
  - [Credits](#credits)
    - [AI Assistance](#ai-assistance)
    - [Research](#research)
  - [License](#license)

## Summary

**Author**: Duc Phan

**Description**

A command-line tool that extracts Word (`.docx`) files into their OOXML representation, allowing you to explore and edit the internal structure of Word documents programmatically. This is particularly useful for understanding and fixing issues that aren't visible through the Word UI.

The extraction approach is inspired by this [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=yuenm18.ooxml-viewer).

## Features

The major features of this program include:

- ✅ **Extract Word files** into a new folder containing their OOXML representation
- ✅ **Summarize structure**: Analyze file count, images, custom XML, and other metadata
- ✅ **Re-zip modified files** back into the original Word file format
- ✅ **Edit custom XMLs** via JSON interface
- ✅ **File watcher** for live updates when files change
- ✅ **Watch for OOXML changes** to update the actual Word file live

**Future Goals** (may not be included to ensure deadline):

- ⏳ Validate OOXML structure

## Motivation

I work with Word files daily where we inject content from different sources into Word files. One of the pain points is the lack of tools to explore and edit the internal structure of those files to understand the output better, as well as fixing bugs that are not visible through the Word UI.

This project serves as the starting point for tools that will be used in my work.

## Prerequisites

- **Rust** (latest stable version recommended)
  - Install from [rustup.rs](https://rust-lang.org/tools/install/)
  - Verify installation: `rustc --version`

## Installation

1. Clone or download this repository
2. Navigate to the project directory
3. Run the program:
   ```bash
   cargo run
   ```

## Usage

### Quick Start

1. Navigate to the project folder in your terminal
2. Run `cargo run`
3. Choose one of the available options from the menu
4. Follow the prompts

> **Note**: For file paths, use paths relative to the root of the project. For example, if you have a `.local/test file.docx` at the root, your input path will be exactly `.local/test file.docx` (no quotes needed).

### Feature Guide

#### 1. Word File Extraction

Extract a Word file into its OOXML structure.

**Input**: Relative path to the Word file (e.g., `.local/document.docx`)

**Output**: A folder with the same name (minus extension) containing:

- `extracted/` - The unzipped OOXML structure

**Example**:

```
Input: .local/my-document.docx
Output: .local/my-document/extracted/
```

#### 2. Re-zip Extracted Folder

Re-zip a modified extracted folder back into a Word file.

**Input**:

- Path to the extracted folder
- Output file name

**Output**: A `.docx` file ready to be opened in Word

**Note**: The output file should match the original format before extraction.

#### 3. Summarize Structure

Analyze and summarize the Word file structure.

**Input**: Path to the source Word file

**Process**:

- Unzips the file if needed
- Analyzes the extracted folder
- Generates a summary report

**Output**: `summary.json` inside the root of the unzipped Word folder

**Summary includes**:

- Basic file info: name, size, number of entries, metadata
- Image count and sizes
- Number of custom XMLs
- Other structural information

#### 4. Analyze Custom XML

Extract and analyze custom XML metadata from Word files.

**Input**: Path to the source Word file

**Process**:

- Unzips the file if needed
- Iterates over the `customXml` folder inside the extracted folder
- Parses `item*.xml` files

**Output**: `customXml.json` inside the root of the unzipped Word folder

**Format**: JSON representation of custom XML tags, attributes, and content

#### 5. Edit Custom XML

Sync edited custom XML back to the Word file structure.

**Prerequisites**:

- Custom XML must have been analyzed first (feature #4)
- `customXml.json` must exist in the root folder

**Input**: Path to the root folder containing:

- The `extracted` folder (unzipped Word file)
- The `customXml.json` file

**Process**:

- Reads and parses `customXml.json`
- Iterates over the `customXml` folder inside the `extracted` folder
- Updates individual XML files correspondingly

**Note**: Only changed custom XML files are updated (not all files).

#### 6. Watch for Changes

Monitor file changes and prompt for automatic updates.

**Input**: Path to the project folder (from the unzip feature)

**Behavior**:

- Watches for file changes until terminated (Ctrl+C)
- Monitors:
  - `customXml.json` changes → prompts to resync custom XML
  - Files in `extracted` folder → prompts to rezip back to Word file

**Example Workflow**:

1. Start the watcher on your extracted folder
2. Edit `customXml.json` in your editor
3. Program detects change and prompts: "Do you want to resync? (y/n)"
4. If yes, updates the XML files in `extracted` folder
5. Edit a file in `extracted` folder
6. Program detects change and prompts: "Do you want to rezip? (y/n)"
7. If yes, creates the updated `.docx` file

## Sample Data

The project includes sample Word files in the `sample_data/` directory to help with testing and evaluation. These files are particularly useful for testing custom XML functionality, which can be difficult to test without proper sample files.

### Available Sample Files

The `sample_data/` directory contains the following test files:

- **`normal.docx`** - A standard Word document without custom XML or special features

  - Use for: Basic extraction, summarization, and re-zipping tests

- **`sample_with_custom_xml.docx`** - A Word document containing custom XML metadata

  - Use for: Testing custom XML analysis (feature #4) and editing (feature #5)
  - Contains: Multiple custom XML items with various structures

- **`sample_with_pictures.docx`** - A Word document with embedded images

  - Use for: Testing image extraction and media handling in summarization

- **`sample_with_custom_xml_and_pictures.docx`** - A Word document with both custom XML and images
  - Use for: Comprehensive testing of all features together
  - Contains: Custom XML metadata and embedded images

### Testing Custom XML Features

Custom XML functionality can be challenging to test without proper sample files. The sample files provided make it easier to:

1. **Test Custom XML Analysis**:

   ```
   Input: sample_data/sample_with_custom_xml.docx
   Output: sample_data/sample_with_custom_xml/customXml.json
   ```

   This will generate a JSON file containing all custom XML data, making it easy to verify the analysis works correctly.

2. **Test Custom XML Editing**:

   - First, analyze the custom XML (feature #4)
   - Edit the generated `customXml.json` file
   - Use feature #5 to sync changes back to the Word file
   - Re-zip the file (feature #2) to create an updated `.docx`

3. **Test File Watcher**:
   - Extract a sample file with custom XML
   - Start the file watcher (feature #6) on the extracted folder
   - Edit `customXml.json` to see live sync in action

### Quick Test Workflow

Here's a recommended workflow for testing with sample data:

```bash
# 1. Extract a sample file with custom XML
Input: sample_data/sample_with_custom_xml.docx

# 2. Analyze the custom XML
Input: sample_data/sample_with_custom_xml.docx
Output: sample_data/sample_with_custom_xml/customXml.json

# 3. (Optional) Edit customXml.json in your editor

# 4. Sync custom XML changes back
Input: sample_data/sample_with_custom_xml/

# 5. Re-zip the modified folder
Input: sample_data/sample_with_custom_xml/extracted
Output: sample_data/sample_with_custom_xml_modified.docx
```

These sample files are especially helpful for grading and evaluation, as they provide a consistent baseline for testing all features, particularly the custom XML analysis and editing capabilities.

## Feature Details

### Word File Structure

A Word file (`.docx`) is essentially a ZIP archive containing:

- **`document.xml`** - The main OOXML representing the actual UI shown in Word
- **Relationship files** - Indicate structural relationships between UI elements
- **Images** - Embedded images and media
- **Custom metadata (`customXML`)** - Programmatically added metadata (found via `item1.xml`, `itemProps1.xml`, `item2.xml`, etc.)
- **Other metadata** - Styles, themes, settings, etc.

This tool helps you unzip, explore, edit, and re-zip this structure.

### User Flow

![User flow](./user-flow.png)

## Project Structure

```
final-project/
├── src/
│   ├── main.rs                 # Entry point
│   └── utils/
│       ├── analyze_custom_xml/ # Custom XML analysis
│       ├── file_watcher/       # File change monitoring
│       ├── files.rs            # File utilities
│       ├── input_utils/        # User input handling
│       ├── print_utils.rs      # Output formatting
│       ├── summarize/          # Structure summarization
│       ├── sync_custom_xml/    # Custom XML synchronization
│       ├── types.rs            # Type definitions
│       └── zip_utils/          # ZIP extraction/compression
├── Cargo.toml                  # Project dependencies
├── preference.json             # User preferences (auto-generated)
└── README.md                   # This file
```

## Known Issues

- Function closure challenges
- Inheritance when serializing structs to JSON
- Parsing string content into JSON
- File watcher may fire multiple events for the same file change (debouncing implemented to mitigate this)

## Credits

### AI Assistance

I used AI to help with:

- Understanding Word file extraction process (spoiler: it's exactly the same as `.zip` extraction)
- Code review after Clippy suggestions
- Folder structure organization for splitting code into modules
- Generating various regex patterns
- Generating unit test cases
- Refactoring features into their own subfolders
- Making input prompts more beautiful ❇️❇️🌟🌟✨✨
- Refind this readme ❇️❇️🌟🌟✨✨
- Randomize custom XML data 🐕🐶🦮🐕‍🦺🐩🦴🌭

### Research

I also researched:

- How to unzip files in Rust
- How to `readdir` in Rust
- How to serialize JSON and write to files in Rust
- How to use Regex in Rust
- How to watch for file changes in Rust

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
