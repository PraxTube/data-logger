# **D**ata L**ogg**er

A CLI tool that allows you to log data manually.
The main use case is to log data that you cannot monitor
any other way and to store it inside a `csv` file.

TODO: Video here

## Usage

The setup is little bit cumbersome, but once
it is finished logging data is very fast and easy.

To setup the default `config.json` and the neccessary
directories, run

```
cargo install dogg
dogg
```

This will generate `~/.config/dogg/data/config.json`
with the following content

```
{
  "dummy": {
    "help": [
      "Your help message here",
    ],
    "type": [
      "u32"
    ],
    "value": [
      "123"
    ]
  }
}
```

You can add new categories with their
parameters here. The supported types are

```
u32
i32
f32
bool
String
```

The `value` will determine the default value,
which will be applied if the user provides
blank input. You can also disable the default
value by using `"None"`.
