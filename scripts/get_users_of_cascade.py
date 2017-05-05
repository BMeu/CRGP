import json
import sys

if __name__ == '__main__':
    # Get the cascade file.
    if len(sys.argv) != 3:
        print('Usage: {name} [CASCADE.json] [OUTPUT-FILE]'.format(name = sys.argv[0]))
        sys.exit(1)
    input_file_path = sys.argv[1]
    output_file_path = sys.argv[2]

    # Extract all retweeting users and write them to the output file.
    with open(input_file_path) as cascade_file:
        with open(output_file_path, 'w') as output_file:
            for line in cascade_file.readlines():
                retweet = json.loads(line)
                output_file.write(str(retweet['user']['id']) + '\n')
