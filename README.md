# rusty-diary
Just a CLI diary in Rust

### Installation
Use the following commands:
1. `git clone https://github.com/mbalamat/rusty-diary`
2. `cd rusty-diary`
3. `cargo build --release`

Step 3 will compile the code into a single binary located in `./target/release/rusty-diary`
Copy that binary in a directory that's in your path.
Use `echo $PATH` in order to find the directories that are in your **$PATH**.

eg. `/usr/local/bin` is in my $PATH so I'll copy the binary there as such:

4. `cp ./target/release/rusty-diary /usr/local/bin/diary`

You should be able now to use `diary`

### Usage
`diary <whatever you want to add to you diary goes here>`

eg. `diary Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.`

Currently, in order to see your past diary entries use `cat` and `jq`.
`cat ~/.rusty-diary/data.json|jq`

the output will be something like:
```
[
  {
    "datetime": "2021-02-21T16:06:16.828983",
    "entry": "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
  }
]
```
