import re
import sys

if __name__ == '__main__':
    # Get the cascade file.
    if len(sys.argv) != 4:
        print('Usage: {name} [INPUT-FILE] [RETWEETS] [OUTPUT-FILE]'.format(name = sys.argv[0]))
        sys.exit(1)
    input_file_path = sys.argv[1]
    num_retweets = float(sys.argv[2])
    output_file_path = sys.argv[3]

    with open(input_file_path) as stats:
        with open(output_file_path, 'w') as output_file:
            interesting_stats = False

            for line in stats.readlines():
                line = line.replace('\n', '')
                go_on = False
                if line.startswith('Iteration with Insertion and Containment Check'):
                    interesting_stats = True
                    go_on = True
                elif line.startswith('Iteration'):
                    interesting_stats = False
                    go_on = True

                add_info = ''
                if interesting_stats and not go_on:
                    trimmed = re.sub(' +', ' ', line)
                    parts = trimmed.split(' ')
                    if len(parts) >= 5:
                        median = float(parts[4].replace(',', ''))
                        secs = median / 1000000000.0
                        rts = int(num_retweets / secs)
                        add_info = '    ' + str(rts) + ' RT/s'
                output_file.write(line + add_info + '\n')
