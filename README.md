# Brochurer

 Calcultor for the page numbers to use for brochure printing.

## Example

To print a brouchure out of a 40 pages document it's recommended to use atleast 2 chunks, if not the brochure will be difficult to bind.
The program output is:

```text
brochurer calc 40 2
Chunk: 1
Duplex: 20,1,2,19,18,3,4,17,16,5,6,15,14,7,8,13,12,9,10,11
  Front: 20,1,18,3,16,5,14,7,12,9
  Back: 2,19,4,17,6,15,8,13,10,11
Chunk: 2
Duplex: 40,21,22,39,38,23,24,37,36,25,26,35,34,27,28,33,32,29,30,31
  Front: 40,21,38,23,36,25,34,27,32,29
  Back: 22,39,24,37,26,35,28,33,30,31
  ```

Open the print dialog in your program and in the `Page Range` insert the first chunk string, corresponding to your printer capabilities (use `Duplex` if your printer support double side printing, `Front` and then `Back` if not). After the print finished, fold the printed paper in half. Next, print the second chunk, using the same procedure. Then, bind the two chunks toghether, and now you have a brochure!

## Usage

This program comes with two commands, `calc` and `build-complete`. The `buld-complete` command build the autocompletition file for your shell to use with this program. The `calc` command is used to calculate the page numbers. The command `brochure help calc` will give this output:

```text
Calculate page numbers

Usage: brochurer calc <PAGE_COUNT> [CHUNK] [OFFSET]

Arguments:
  <PAGE_COUNT>  Page count
  [CHUNK]       Chunk number [default: 1]
  [OFFSET]      Page numbering offset [default: 0]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
