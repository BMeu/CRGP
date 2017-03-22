# CRGP

A graph-parallel approach to reconstructing the influences within Retweet cascades.

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

## File Formats

`CRGP` requires two input files: a list of friends for each user and the retweets.

### Friends

The friends file is currently a text file, on each line specifying a user followed by a list of all their friends. Each
user and friend is given by their user ID. For an example, see [`data/friends_test.txt`](data/friends_test.txt).

### Retweets

The retweet file is a list of JSON-encoded Retweets, each Retweet on a new line. It may contain Retweets from multiple
cascades. For an example, see ['data/cascade_test.json'](data/cascade_test.json).

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
