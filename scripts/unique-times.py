import json

if __name__ == '__main__':
    # Get the cascade file.
    if len(sys.argv) != 2:
        print('Usage: {name} [CASCADE.json]'.format(name = sys.argv[0]))
        sys.exit(1)
    input_file_path = sys.argv[1]

    timestamps = dict()
    number_of_lines = 0

    with open(input_file_path) as retweets:
        for line in retweets:
            retweet = json.loads(line)
            occurrences = timestamps.get(retweet['created_at'], 0)
            occurrences += 1
            timestamps[retweet['created_at']] = occurrences

            number_of_lines += 1

    occurrences = dict()
    for timestamp, occurrence in timestamps.items():
        number = occurrences.get(occurrence, 0)
        number += 1
        occurrences[occurrence] = number

    print(occurrences)

    print('Retweets: {number}'.format(number = number_of_lines))
    print('Unique Timestamps: {number}'.format(number = len(timestamps)))
    print('Duplicate Timestamps: {number}'.format(number = number_of_lines - len(timestamps)))
