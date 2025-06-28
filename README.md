# ned

**ned** (new ed) is a minimal, line-based text editor written in Rust, modeled on the classic Unix `ed`. It prints a startup cheat sheet, supports core line-editing commands, and writes changes back to disk. No syntax highlighting.

---

## Version

v0.1

## Features

* **Append** text after a line: `[address]a`
* **Insert** text before a line: `[address]i`
* **Change** lines in a range: `[range]c`
* **Delete** lines in a range: `[range]d`
* **Print** lines in a range: `[range]p`
* **Write** buffer back to file: `w`
* **Quit** editor: `q`
* Addresses:

  * A line number (e.g. `1`)
  * `$` for the last line
  * Range `start,end` (e.g. `1,5`)
  * Omitted defaults to the entire buffer (`1,$`)
* **Multi‑line input** for `a`, `i`, `c` ends on a single `.` line
* **Creates the file** if it doesn’t exist

---

## Installation

```bash
git clone https://github.com/chasekbryan/ned.git
cd ned
cargo build --release
```

The compiled binary will be at `./target/release/ned`.

---

## Usage

```bash
./target/release/ned <filename>
```

On launch, you’ll see the cheat sheet:

```text
ned v0.1 - Commands:
  [address]a      Append text after address
  [address]i      Insert text before address
  [range]c        Change lines in range
  [range]d        Delete lines in range
  [range]p        Print lines in range
  w               Write buffer to file
  q               Quit editor
```

Then enter commands **without** a leading colon:

```text
1,$p         # Print all lines
2a           # Append after line 2
New line     # (type your text)
.            # End insert mode
w            # Write changes to file
q            # Quit
```

### Command Reference

| Command    | Description                     |
| ---------- | ------------------------------- |
| `[addr]a`  | Append after the given address  |
| `[addr]i`  | Insert before the given address |
| `[range]c` | Change lines in the given range |
| `[range]d` | Delete lines in the given range |
| `[range]p` | Print lines in the given range  |
| `w`        | Write buffer back to the file   |
| `q`        | Quit the editor                 |

*Address* can be a number (1), `$` (last line), or omitted (defaults to `1,$`).
*Range* is two addresses separated by a comma (e.g. `3,5`).

---

## Future Improvements

* Substitute command (`s/pattern/replacement/`)
* Shell escapes (`!cmd`)
* Undo/redo support
* Multi-file editing

---

Enjoy using **ned**!
