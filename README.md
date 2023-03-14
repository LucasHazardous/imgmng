# imgmng

To build an executable:

```
cargo build --release
```

# Usage

## Selecting files

By default working directory is the directory where from which the program was launched.

There are three ways of selecting files to work on:

* specifying working directory and then individual file names

```
imgmng /home/lucas/workingpath/ file1.jpg file2.png
```

* answering yes to the first question after running the program with just working directory or no arguments at all to work on all images

* manually selecting images to work on in working directory by saying *n* to first question

### How to use file selector?

* <kbd>w</kbd> - go one entry up
* <kbd>s</kbd> - go one entry down
* <kbd>e</kbd> - select/unselect current entry (selected entries will be highlighted with green color)
* <kbd>a</kbd> - finish selecting files and proceed

---

## Applying modifications

After selecting files to work on respond to questions about modifications, these questions will include:

* resize
* color invert
* special effect

When the modifications are selected the processing part beings where you will be notified on the progress in real time. All modified images will be saved to a directory called *modified* which will be located in your working directory.