# BigBrainFuck
Bigger and Better

The main difference between bbf and bf is: in bbf, code and data is the same
The code is stored in the memory array
This means you can manipulate code during the runtime, creating all sorts of programs in fewer lines.

## Operators:
* `+`: increment the data in current data-cell
* `-`: decrement the data in current data-cell
* `>`: move the data pointer to right
* `<`: move the data pointer to left
* `.`: print the byte in current data-cell
* `,`: getch input and save to current data-cell
* `[`: start a loop, if current data-cell is not 0
* `]`: end loop
And some new ones...
* `$`: end the program
* `/`: increment the data pointer to right by 4
* `\`: increment the data pointer to left by 4
* `#`: set the code pointer to the data pointer
* `%`: set the data pointer to the code pointer
* `(` and `)`: comment
* `&`: push the current data pointer to pointer stack
* `*`: pop the pointer stack and set data pointer to the pointer
* `{`: push the current data-cell data to the cell stack
* `}`: pop the cell stack and set data to data in current data-cell
And some more...
* `1`: set the data pointer to the data in current data-cell
* `2`: set the code pointer to the data in current data-cell
* `3`: increment the data pointer by the data in current data-cell
* `4`: increment the code pointer by the data in current data-cell
* `5`: set the data in current data-cell to data pointer % 256
* `6`: set the data in current data-cell to code pointer % 256