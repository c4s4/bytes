# Bytes

Tool to convert bytes array to string and string to bytes array.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *bytes* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/bytes/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/bytes/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *bytes* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/bytes/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *bytes*.

## Usage

Convert bytes array to string:

```bash
$ bytes "[65, 66, 67]"
ABC
```

Its also works with hexadecimal:

```bash
$ bytes "[0x41, 0x42, 0x43]"
ABC
```

Convert string to bytes array:

```bash
$ bytes "ABC"
[65, 66, 67]
```

*Enjoy!*
