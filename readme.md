# Yeller

Ever felt old? Well yeller is for you! Yeller enforces set rules and has innovative syntax, allowing for quicker reading and understanding of code, such as:

- a maxmium of 4 letter long variable names
- a 1/5 chance your program will fail to compile
- lonely variables
- etc.

## Installation

Make sure you have python 3 and have pip-installed the "gtts" library.  
You will also need llvm-14 and clang installed.

## Usage

```bash
yeller <input-file> <output-file>
```

This will compile to your target, which is the computer you compile it on.

## Examples

Hello, world!:

```text
the function numbered 1 is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits shall be equal to main left_bracket right_bracket left_parenthesis
i shall invoke the function named putchar and it shall take the parameters left_brace 72 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 101 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 108 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 108 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 111 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 44 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 32 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 87 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 111 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 114 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 108 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 100 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 33 right_brace period
i shall invoke the function named putchar and it shall take the parameters left_brace 10 right_brace period
return 0 period
right_parenthesis
the function numbered 2 is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits shall be equal to putchar left_bracket c is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits right_bracket semicolon
```

Fibonacci:

```text
the function numbered 1 is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits shall be equal to main left_bracket right_bracket left_parenthesis
return i shall invoke the function named fib and it shall take the parameters left_brace 5 right_brace period
right_parenthesis
the function numbered 2 is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits shall be equal to fib left_bracket count is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits right_bracket left_parenthesis
i am declaring a variable named friendly is integer_meaning_whole_in_latin_with_exactly_thirty_two_bits shall be equal to 0 period
in the case that count is zero do return 1 period
in the case that left_brace count minus 1 right_brace is zero do return 1 period
return i shall invoke the function named fib and it shall take the parameters left_brace count minus 1 right_brace plus i shall invoke the function named fib and it shall take the parameters left_brace count minus 2 right_brace period
right_parenthesis
```

## Warning

I made this in less than 24 hours. I think this is obvious but you should never, ever, ever make any real programs with this. There are so many bugs it's actually crazy.
