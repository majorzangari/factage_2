# Factage
WIP
should prolly add an actual description of what it does cuz rn this is all meaningless
## Symbols
### Values
- `0-9` are integer values.
- `a-z` and `A-Z` are characters.
- `"` is space-bar.
- `\` is a new-line (`\n`) character.
- `;` ends the program when attempting to output to console.
### Arithmetic
#### Binary Operators
All binary operators take the values from directly to their left and directly to their right and outputs the result directly below.
- `+` outputs the sum of the values.
- `-` outputs the difference between the variables.
- `*` outputs the product of the values.
- `/` outputs the integer division quotient below.
- `%` outputs the modulus result of the values.
- `&` outputs the bitwise AND of the values.
- `|` outputs the bitwise OR of the values.
- `=` outputs a `1` if both values are equal or a `0` otherwise.
- `<` outputs a `1` if the left value is less than the right value or a `0` otherwise.
- `>` outputs a `1` if the left value is greater than the right value or a `0` otherwise.
#### Unary Operators
All unary operators take the value directly to the left and outputs the result directly to the right.
- `!` inverts the value, converting zero to one and all other values to `0`, and outputs result.
- `:` duplicates and outputs the value, does not delete initial value.
### Movement
- ` ` any values not on other symbols fall down at a rate of 1 unit per tick.
- `,` shifts the value two units downwards.
- `}` shifts the value one unit to the right.
- `]` shifts the value two units to the right
- `{` shifts the value one unit to the left.
- `[` shifts the value two units to the left.
- `^` shifts the value one unit upwards.
- `'` shifts the value two units upwards.
- `?` takes the value directly above it and shifts the value based on the value directly below it. If the value below is a `0`, the value is shifted directly to the right of the `?`, if it is a value other than zero, it is shifted to the left. Does not delete the below value.
### Other
- `@` outputs any values on it to the console. Does not delete the values. Can only have one `@` in the program.
- `#` deletes any values on it.
- `_` prevents values from going through it. Values can atop it without falling.
## Interpreter
To run a program, invoke the interpreter with the source code's file name as a command-line argument, e.g.
```
java Factage program.txt
```
