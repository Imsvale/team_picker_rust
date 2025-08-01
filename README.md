## About This Project

This is a full translation of [arkadye's original project](https://github.com/arkadye/team_picker) into Rust for my own learning purposes.

The original project was developed in C++ and is licensed under the MIT License. This project is also released under the MIT License.

All credit to arkadye for the original implementation and logic.

## Usage
- Copy player data from your team page on the Brutalball website.
- Paste into `team_data.txt`.
- Run the program.

### Command line parameters


## 🧮 Supported Expression Syntax

Formulas used in `composition.txt` can include stat variables, arithmetic, logic, comparisons, and built-in functions. All expressions are evaluated per player using their stats from `team_data.txt`.

### 🧑‍💻 Stat Variables

You can reference any stat column defined in `team_data.txt`. Examples:

- `SPD`, `STR`, `DUR`, `TCK`, `QB`, `HB`, etc.
- Stat variables are **case-insensitive**.

### 🔢 Arithmetic Operators

| Operator | Meaning        | Example            |
|----------|----------------|--------------------|
| `+`      | Addition       | `QB + HB`          |
| `-`      | Subtraction    | `STR - SPD`        |
| `*`      | Multiplication | `DUR * 0.5`        |
| `/`      | Division       | `STR / 2`          |
| `^`      | Power          | `2 ^ 3` → `8`      |
| `-x`     | Negation       | `-QB`              |

### 🔍 Comparison Operators

| Operator | Meaning                  | Example             |
|----------|--------------------------|---------------------|
| `==`     | Equal to                 | `SPD == 10`         |
| `=`      | Equal to (same as above) | `SPD = 10`          |
| `!=`     | Not equal to             | `STR != 20`         |
| `>`      | Greater than             | `QB > HB`           |
| `>=`     | Greater than or equal to | `STR >= 25`         |
| `<`      | Less than                | `SPD < 15`          |
| `<=`     | Less than or equal to    | `SPD <= 10`         |

- All comparisons return `1.0` for true, `0.0` for false. 
- This means you can do logic with the arithmetic operators `+` and `*` as well:
  - Any resulting **non-zero** value is considered **true**.
  - `1.0 + 0.0 = 1.0` → `true`
  - Equivalent to `OR(1.0, 0.0) = OR(true, false) = true`
  - `1.0 * 0.0 = 0.0` → `false`
  - Equivalent to `AND(1.0, 0.0) = AND(true, false) = false`

### 🧠 Logical Operators

| Operator | Meaning     | Example                      |
|----------|-------------|------------------------------|
| `!`      | NOT         | `!(SPD > 10)`                |
| `&&`     | AND         | `(SPD > 5) && (STR > 5)`     |
| `\|\|`     | OR          | `(SPD > 10) \|\| (STR > 10)`   |

You can also use equivalent function calls: `NOT(x)`, `AND(x, y)`, `OR(x, y)`

### 🧮 Built-in Functions

Function names are **case-insensitive**.

| Function             | Description                                   | Example                            |
|----------------------|-----------------------------------------------|------------------------------------|
| `MIN(a, b, ...)`     | Returns the minimum of all values             | `MIN(QB, HB)`                      |
| `MAX(a, b, ...)`     | Returns the maximum of all values             | `MAX(Spd, Str, Dur)`               |
| `AVERAGE(a, b, ...)` | Returns the average (mean) of all values      | `AVERAGE(Spd, Str, Tck)`           |
| `POW(base, exp)`     | Raises `base` to the power of `exp`           | `POW(2, 3)` → `8`                  |
| `IF(cond, a, b)`     | Returns `a` if `cond` is true, otherwise `b`  | `IF(STR > SPD, STR, SPD)`          |
| `NOT(x)`             | Logical NOT                                   | `NOT(0)` → `1`, `NOT(1)` → `0`     |
| `AND(x, y)`          | Logical AND                                   | `AND(SPD > 5, STR > 5)`            |
| `OR(x, y)`           | Logical OR                                    | `OR(SPD > 10, STR > 10)`           |

### 🧼 Whitespace

Whitespace is removed before parsing, and effectively ignored: `QB + HB` is equivalent to `QB+HB`.
