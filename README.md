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
$ cargo run --release -- data/social_graph data/retweets.json
```

## Distributed Computation

The social graph might quickly get too large to fit into a single computer's main memory. This problem can be
circumvented by distritubting the computation among several machines. Let's look at an example:

Assume we have got three machines among which we want to distribute our computation; their hostnames are `raspberry`,
`blackberry`, and `blueberry`. On each of these computers we will be running one process of `CRGP`, thus three in total.
Each process of `CRGP` can use multiple threads for the computation, so-called *workers*, but all processes must use
the same number of workers. Since our weakest machine has got four CPU cores, we will tell each process to use four
workers.

We also have to tell each process where to find the other processes. For this, we create a text file `hosts.txt` with
the same content on all three machines. On each line it gives the address (either the hostname or the IPv4-address) of
each computer and the port used for communication, separated by a colon: 

```text
raspberry:2101
blackberry:2101
blueberry:2101
```

Each process has a unique ID `n` in the interval `[0, N-1]` (where `N` is the total number of processes), corresponding
to the line numbers in the host file: the process running on the host specified in the first line gets ID `0`, the one
on line two gets ID `1`, and so on. 

On each of the machines, we can now start `CRGP` with the following command, where in accordance with our hosts file
`[n]` is `0` on `raspberry`, `1` on `blackberry`, and `2` on `blueberry` (line breaks are only added to improve
legibility and must be omitted when running the command):

```bash
$ cargo run --release --
    --process [n]
    --processes 3
    --workers 4
    --hostfile hosts.txt
    [FRIENDS]
    [RETWEETS]
```

`[FRIENDS]` and `[RETWEETS]` are of course paths to the respective data sets. Only process `0` actually accesses these,
that is, for all other processes any other path (even an invalid one) can be given (but they must be given).

If you do not specify a host file but use `N > 1` processes, `CRGP` will automatically use `localhost` on the ports
`2101` through `2101 + N - 1` as hosts. This way, you can easily test the distributed computation on a single machine.

## File Formats

`CRGP` requires two input files: a list of friends for each user and the retweets.

### Friends

`CRGP` expects the friends for each user in a CSV file, each user in a defined directory structure within a TAR archive,
and each TAR archive in a defined directory structure.

Each CSV file contains all the friends (one per line) of a single user. The first line of a file may contain meta data
about the user in the following format:

```text
[Name];[ID];[#Followers];[#Friends];[#Statuses]
```

I.e., a semicolon-separated list of the user's screen name, their user ID, how many followers they have, how many
friends they have, and how many Tweets they wrote. Note that the number of friends in the meta data is allowed to differ
from the amount of friends actually specified below; this can be used to reduce the size of social graph if its
subset is known for a cascade.

The ID (`[ID]`, must be parsable to `u64`) of a user is encoded into the filename and the directory path:

 * Filename: `friends[ID].csv`
 * Directory path (without TAR archive): `[ID]` is padded with leading zeroes to twelve digits, then broken into a path
   with chunks of size three.

Within each top-level folder, the sub-directories are grouped by their first two digits and packed inside a TAR archive
with those two digits as file name.

For example:

 * The friends of user `42` are stored in `/000/000/friends42.csv` within `000/00.tar`.
 * The friends of user `1337` are stored in `/000/001/friends1337.csv` within `000/00.tar`.
 * The friends of user `420001000024` are stored in `/001/000/friends420001000024.csv`. within `420/00.tar`

For a full example (with some invalid files for testing), see [`data/social_graph`](data/social_graph).

### Retweets

The retweet file is a list of JSON-encoded Retweets, each Retweet on a new line. It may contain Retweets from multiple
cascades. For an example, see [`data/retweets.json`](data/retweets.json).

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

* Invalid directory and file names within the social graph directory and TAR files.
* Creation of result and statistics files.
* Per-user information on the user's actual number of friends, the given number of friends, and the possibly created
  fake friends.
* In-progress information for each processed batch of Retweets.

**Info:** `-vvv`

* Algorithm parameters (e.g. batch size, data sets, number of processes and workers, ...).
* The following algorithm execution events:
    * Starting and finishing to load the social graph.
    * Starting and finishing to load the Retweet file into memory.
    * Starting and finishing to process the Retweets.
* Overall information on the actual number of friends in the social graph, the given number of friends, and the possibly
  created fake friends.

**Warn:** `-vv`

* Parse failures: user IDs, Retweets.
* Encountering users without any friends when loading the social graph.
* Encountering input (e.g. in files) that is not valid UTF-8.

**Error:** `-v`

* Failures during I/O operations.

## Author

`CRGP` is developed by [Bastian Meyer](http://www.bastianmeyer.eu/)
<[bastian@bastianmeyer.eu](mailto:bastian@bastianmeyer.eu)> for his master's thesis at the
[Research Group on Web Science](https://websci.informatik.uni-freiburg.de/),
[University of Freiburg, Germany](https://www.uni-freiburg.de), under Prof. Dr. Peter Fischer.

## License

`CRGP` is licensed under either of

 * Apache License, Version 2.0, ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
