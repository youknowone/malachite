import os

MAX_LINE_LENGTH = 100

line_length_exceptions = set((
    # long Markdown table rows and/or links
    ('./malachite-base/src/lib.rs', 65),
    ('./malachite-base/src/num/arithmetic/mod.rs', 333),
    ('./malachite-base/src/num/arithmetic/mod.rs', 334),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1334),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1574),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1575),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1576),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1577),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 85),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 244),
    ('./malachite-base/src/num/arithmetic/round_to_multiple_of_power_of_2.rs', 118),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 154),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 156),
    ('./malachite-base/src/num/exhaustive/mod.rs', 1084),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 452),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 641),
    ('./malachite-float/src/conversion/mod.rs', 225),
    ('./malachite-float/src/lib.rs', 24),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 35),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 36),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 37),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 72),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 73),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 84),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 85),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 86),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 115),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 117),
    ('./malachite-nz/src/lib.rs', 36),
    ('./malachite-nz/src/lib.rs', 103),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 42),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 43),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 44),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 146),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 147),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 170),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 171),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 172),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 555),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 557),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 529),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 531),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 834),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 836),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 323),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 508),
    ('./malachite-nz/src/natural/conversion/mod.rs', 257),
    ('./malachite-q/src/arithmetic/mod.rs', 79),
    ('./malachite-q/src/arithmetic/mod.rs', 81),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 145),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 232),
    ('./malachite-q/src/lib.rs', 54),
))

def lint(filename):
    i = 1
    with open(filename) as f:
        for line in f.readlines():
            line = line.rstrip()
            is_exception = (filename, i) in line_length_exceptions
            if is_exception:
                if len(line) <= MAX_LINE_LENGTH:
                    raise ValueError(f'line not too long: {filename}: {i} {line}')
            elif len(line) > MAX_LINE_LENGTH:
                raise ValueError(f'line too long: {filename}: {i} {line}')
            i += 1
    return i - 1

filename_list = []
for root, directories, filenames in os.walk('.'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and '.history' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

line_count = 0
for filename in filename_list:
    line_count += lint(filename)
print(f'{line_count} lines checked')
