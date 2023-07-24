# Oxidized-xml

Oxidized-xml is a project about making generic XML toolkit in which to query, transform, validate and edit 
XML documents but in the Rust programming language. 

The idea for the architecture is to have the core library with "all" of the functionality and then attach different ways to interact with the 
core functionalities. Initially there is a command-line interface (CLI) in order to access the core functions,
and the initial core functionality is to fetch values inside the XML structure queried with an XPath. 

This project will hopefully expand over time and provide as a playground to learn Rust. 
Feel free to fork, experiment or contribute in any way.

## Prerequisite
You'll need the rust tool-chain. 
The minimal supported rust version is 1.64.0 due to dependencies.

## How to build
If you are unfamiliar with the rust programming language then please read the documentation at:
* https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project

## Try it out
### Examples
#### The CLI
1. The following example is to show how the CLI can be utilised given that you are positioned at the root of the project:
    ```bash
   cd oxidized-xml-cli
   cargo run -- --xpath "//*[@id='42007']/target" -- ../test/data/sma_gentext.xml
    ```
    The --xpath flag sets the query of the element targeted and the current functionality can only output the elements value.
    The XPath-query itself will find any element with the name target and a parent with the attribute "id" 
    that has the value of "42007". 
    This above example will yield the following output;
    ```
    Obs!
    ```
    
    