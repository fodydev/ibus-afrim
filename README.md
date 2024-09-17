# ibus-afrim

Afrim Engine for IBus, a non-language specific Input Method Engine.

Following documents provide further instruction:
- [`COPYING`](COPYING): GPLv3 Licence
- [`INSTALL`](INSTALL): Installation instruction

## State of the project

In development... 👷

### What is this?

`ibus-afrim` is an implementation of the [`Afrim input method`](https://github.com/fodydev/afrim) on the IBus input method framework.

### How to install

Confer [`INSTALL`](INSTALL).

### Usage

**start:**

```
ibus restart
ibus engine afrim
```

**commands:**

- CTRL_LEFT+CTRL_RIGHT to pause / resume the IME.
- CTRL+SHIFT_LEFT to select the previous candidate.
- CTRL+SHIFT_RIGHT to select the next candidate.
- CTRL+SPACE to commit the selected candidate.

### How to report bugs

You can report your issue in our [issue tracker](https://github.com/fodydev/ibus-afrim/issues).

### Credits

Understanding the IBus documentation was a challenge for us. We would like to thank the contributors of the following projects:
- [eei](https://github.com/Mindful/eei) - We extracted the minimal IBUs project in rust from their work.
- [ibus-skk](https://github.com/ueno/ibus-skk) - Their IBus implementation, helped us gain a better understanding of the IBus API.
