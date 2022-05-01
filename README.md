# aspeak

A simple text-to-speech client using azure TTS API(trial).

**TL;DR**: This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you.

## Installation

```sh
$ pip install aspeak
```

## Usage

```
usage: aspeak [-h] [-v] [-t [TEXT] | -s [SSML]] [-f FILE] [-o OUTPUT_PATH]

This program uses trial auth token of Azure Cognitive Services to do speech synthesis for you.

options:
  -h, --help            show this help message and exit
  -v, --version         show program's version number and exit
  -t [TEXT], --text [TEXT]
                        Text to speak. Left blank when reading from file/stdin.
  -s [SSML], --ssml [SSML]
                        SSML to speak. Left blank when reading from file/stdin.
  -f FILE, --file FILE  Text/SSML file to speak, default to `-`(stdin).
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        Output wav file path
```

- If you don't specify `-o`, we will use your default speaker.
- If you don't specify `-t` or `-s`, we will assume `-t` is provided.

### Examples

#### Speak "Hello, world!" to default speaker.

```sh
$ aspeak -t "Hello, world!" -o output.wav
```

#### Save synthesized speech to a file.

```sh
$ aspeak -t "Hello, world!" -o output.wav
```

#### Read text from file and speak it.

```sh
$ cat input.txt | aspeak
```

or

```sh
$ aspeak -f input.txt
```

#### Read from stdin and speak it.

```sh
$ aspeak
```

or (more verbose)

```sh
$ aspeak -f -
```

