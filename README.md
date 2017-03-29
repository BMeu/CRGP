# CRGP

[ ![Codeship Status for BMeu/crgp](https://app.codeship.com/projects/7d2924a0-f1e4-0134-404a-569aa21b12f1/status?branch=master)](https://app.codeship.com/projects/209508)

A graph-parallel approach for reconstructing the influences within Retweet cascades.

## Usage

Using [Rust's package manager `Cargo`](https://www.rustup.rs/), executing `CRGP` is really simple:

```bash
$ cargo run --release -- <FRIENDS> <RETWEETS> 
```

This will compile `CRGP` with all its dependencies and run the binary. A full list of options is available using the
`-h` flag:

```bash
$ cargo run --release -- -h
```

## Example

This repository includes a few data sets you can use to test `CRGP`.

### Test Data

Two tiny Retweet cascades (each has three Retweets) on a tiny social graph.
 
```bash
$ cargo run --release -- -r data/tests/friends.txt data/tests/cascade.json
```

There is also a CSV files version of the social graph.

```bash
$ cargo run --release -- -r data/tests/friends data/tests/cascade.json
```

### Real-Life Data

Two small Retweet cascades from Twitter. The social graph is an extract from the actual Twitter graph.

A single cascade with 3,500 Retweets:

```bash
$ cargo run --release -- data/friends.txt data/cascade3500.json
```

A single cascade with 7,226 Retweets:

```bash
$ cargo run --release -- data/friends.txt data/cascade7226.json
```

## File Formats

`CRGP` requires two input files: a list of friends for each user and the retweets.

### Friends

`CRGP` supports two formats for friends files: a simple text file and a CSV file structure. If the `<FRIENDS>` argument
points to a file, the former format is assumed, if it points to a directory, the CSV files structure will be used.

#### Text File

A single text file, on each line specifying a user followed by a list of all their friends. Each user and friend is
given by their user ID. For an example, see [`data/tests/friends.txt`](data/tests/friends.txt).

The user is separated from their friends by a colon (`:`). The list of friends is comma-separated (`,`). For example, if
user `1` is friends with users `2` and `4`, the line would look like this:

```text
1:2,4
```

#### CSV Files

Multiple CSV files in a defined directory structure, each specifying all friends for a user.

Each CSV file contains all the friends (one per line) of a single user. The ID (`[ID]`, must be parsable to `u64`)
of a user is encoded into the filename and the directory path:

 * Filename: `friends[ID].csv`
 * Directory path: `[ID]` is padded with leading zeroes to twelve digits, then broken into a path with chunks of
   size three.

For example:

* The friends of user `42` are stored in `000/000/000/friends42.csv`.
* The friends of user `1337` are stored in `000/000/001/friends1337.csv`.
* The friends of user `420000000024` are stored in `420/000/000/friends420000000024.csv`.

For a full example (with some invalid files for testing), see [`data/tests/friends`](data/tests/friends).

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

## Author

`CRGP` is developed by [Bastian Meyer](http://www.bastianmeyer.eu/)
<[bastian@bastianmeyer.eu](mailto:bastian@bastianmeyer.eu)> for his master's thesis at the
[Research Group on Web Science](https://websci.informatik.uni-freiburg.de/),
[University of Freiburg, Germany](https://www.uni-freiburg.de), under Prof. Dr. Peter Fischer.

`CRGP` is licensed under the MIT license (see [LICENSE.txt](LICENSE.txt) for details).
