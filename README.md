# CRGP

[ ![Codeship Status for BMeu/crgp](https://app.codeship.com/projects/7d2924a0-f1e4-0134-404a-569aa21b12f1/status?branch=master)](https://app.codeship.com/projects/209508)

A graph-parallel approach for reconstructing the influences within Retweet cascades.

## Usage

Using [Rust's package manager `Cargo`](https://www.rustup.rs/), executing `CRGP` is really simple:

```bash
$ cargo run --release -- [FRIENDS] [RETWEETS] 
```

This will compile `CRGP` with all its dependencies and run the binary. A full list of options is available using the
`-h` flag:

```bash
$ cargo run --release -- -h
```

## Example

This repository includes a data set you can use to test `CRGP`. It consists of two tiny Retweet cascades (each with
three Retweets) on a tiny social graph:

```bash
$ cargo run --release -- data/tests/friends-tar data/tests/cascade.json
```

## File Formats

`CRGP` requires two input files: a list of friends for each user and the retweets.

### Friends

`CRGP` expects the friends for each user in a CSV file, each user in a defined directory structure within a TAR archive,
and each TAR archive in a defined directory structure.

Each CSV file contains all the friends (one per line) of a single user. The ID (`[ID]`, must be parsable to `u64`)
of a user is encoded into the filename and the directory path:

 * Filename: `friends[ID].csv`
 * Directory path (without TAR archive): `[ID]` is padded with leading zeroes to twelve digits, then broken into a path
   with chunks of size three.

Within each top-level folder, the sub-directories are grouped by their first two digits and packed inside a TAR archive
with those two digits as file name.

For example:

 * The friends of user `42` are stored in `/000/000/friends42.csv` within `000/00.tar`.
 * The friends of user `1337` are stored in `/000/001/friends1337.csv` within `000/00.tar`.
 * The friends of user `420001000024` are stored in `/001/000/friends420001000024.csv`. within `420/00.tar`

For a full example (with some invalid files for testing), see [`data/tests/friends-tar`](data/tests/friends-tar).

### Retweets

The retweet file is a list of JSON-encoded Retweets, each Retweet on a new line. It may contain Retweets from multiple
cascades. For an example, see [`data/tests/cascade.json`](data/tests/cascade.json).

Each JSON object must contain the following fields (line breaks added for readibility):

```json
{
    "created_at": 987654321,
    "text": "RT @ArthurDent: They say the Ultimate Answer is 42, how am I supposed to know what the question is? Could be anything, I mean, what's 6x7?",
    "id": 2,
    "retweeted_status": {
        "created_at": 123456789,
        "text": "They say the Ultimate Answer is 42, how am I supposed to know what the question is? Could be anything, I mean, what's 6x7?",
        "id": 1,
        "user": {
            "id": 42,
            "screen_name": "ArthurDent"
        },
        "retweet_count": 1
    },
    "user": {
        "id": 1337,
        "screen_name": "ZaphodB"
    },
    "retweet_count": 1
}
```

## Logging

`CRGP` can log certain events. By default, this is disabled. To enable logging to `STDERR`, specify the verbosity level
of the log message by passing the `-v` flag in the desired amount to `CRGP` . 

The log can be written to a file by passing the `-l` option to `CRGP`. It expects a path to the directory where the log
file will be saved.

There are four log levels. In ascending order, they are `Trace`, `Info`, `Warn`, and `Error`, that is, the lower the
level, the more information will be logged. Lower levels include all above them. The following events are logged at the
specified levels.

**Trace:** `-vvvv`

 * Invalid directory and filenames within the directory of the social graph (when using the CSV format).
 * Creation of result and statistics files.

**Info:** `-vvv`

 * Failure to parse user IDs.
 * Failure to parse Retweets.
 * The following events during the algorithm execution:
   * Starting and finishing to load the social graph.
   * Starting and finishing to load the Retweet file into memory.
   * Starting and finishing to process the Retweets (with in-progress information for each batch of Retweets).

**Warn:** `-vv`

 * Encountering users without any friends when loading the social graph.
 * Encountering input (e.g. in files) that is not valid UTF-8.

**Error:** `-v`

 * Failure to read the contents of a directory.
 * Failure to open files.
 


## Author

`CRGP` is developed by [Bastian Meyer](http://www.bastianmeyer.eu/)
<[bastian@bastianmeyer.eu](mailto:bastian@bastianmeyer.eu)> for his master's thesis at the
[Research Group on Web Science](https://websci.informatik.uni-freiburg.de/),
[University of Freiburg, Germany](https://www.uni-freiburg.de), under Prof. Dr. Peter Fischer.

`CRGP` is licensed under the MIT license (see [LICENSE.txt](LICENSE.txt) for details).
