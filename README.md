# OOXML Manipulator

## Summary

**Author**: Duc Phan

**Description**

A program that extracts a Word file into its [OOXML](https://en.wikipedia.org/wiki/Office_Open_XML) representation and also provide some utils

The major features of this program includes

- extract a Word file into a new folder containing its OOXML representation
- summarize the structure: how many files, how many images, how many custom XML, etc
- re-zip into the original Word file after the user has modified the file
- allow adding/editing custom XMLs via a primitive GUI
- validate OOXML structure - might not be include to ensure I meet the deadline

The extraction is inspired by this [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=yuenm18.ooxml-viewer)

## Motivation

I work with Word files daily where we have to inject content from different sources to Word files.
One of the pain points is the lack of tools to explore and edit the internal structure of those file to understand the output better as well as well as fixing bugs that are not visible through the Word UI.

So this project is going to be the starting point for the tools that will be used in my work.

## Context

This section explains the individual features for more info

### User flow

![User flow](./user-flow.png)

### Feature explanation

1. Word file extraction

## How to run

- Navigate your favorite terminal to the project folder
- Run `cargo run`
- Choose one of the options
- See below for instruction on individual options

> Note that for file paths, use the path that is relative to the root of the project. For example, I have a `.local/test file.docx` at the root, my input path will be exactly the same, no quote needed

TODO

## Issue and credit

- I asked ChatGPT to understand what the process is for a Word file extraction (spoiler alert, it's exactly the same as a .zip extraction)
  TODO organize
