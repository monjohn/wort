# Wort

Access a german-english dictionary through the command line

To install, you must have cargo already installed on your computer.


## Directions
1. Make a directory for files and copy dictionary from this folder to that directory

```sh
mkdir ~/.bin # Make sure this is on your classpath, or chose a folder which is already
cp ./resources/de-en.txt ~/.bin/
```

2. Add location to project, by opening the file, `src/main.rs`, in this project and changing the first line of the main function, `let dictionary = "(insert path here)/.bin/de-en.txt";` to be the location of your file.

3. Build the project and copy it to your directory:
```sh
cargo build --release
cp target/release/wort ~/.bin/
```

4. Try it out with
```sh
wort mahnen
```

## Options
```sh
Usage: wort FILE [options]

Options:
    -a, --all           include all occurances of word, not just definition
    -h, --help          print this help menu
    -m, --more          returns occurrences after the first within a
                        definition
    -o, --out           output suitable for appending to a csv
```

## To Be Implemented:
1. accept csv file to append results to
