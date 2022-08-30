# rextract
CLI tool that extracts a regex pattern from a list of urls. 
The tool is written in Rust and supports PCRE.

## Installation

### Step 1:

Visit https://rustup.rs/ and follow the instructions to get started with rust and cargo.

### Step 2: 

```
> cargo install --git https://github.com/iustin24/rextract
```

## Usage

The tool takes a list of urls from stdin and extracts the regex supplied as an argument. 

### Extract HTML Title example ( using lookarounds ):

```
> cat urls.txt
https://www.google.com/
https://youst.in/

> cat urls.txt | rextract '(?im)(?<=<title>).*(?=</title>)'
Google
Youstin
```

### Extract UUIDs

```
> cat urls.txt
https://www.uuidtools.com/docs

> cat urls.txt | rextract '[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}'
b01eb720-171a-11ea-b949-73c91bba743d
b01eb720-171a-11ea-b949-73c91bba743d
b01eb720-171a-11ea-b949-73c91bba743d
b01eb720-171a-11ea-b949-73c91bba743d
```
