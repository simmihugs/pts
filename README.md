# Pts
  A pts file parser

## install 
```
cargo build
cargo build --release
cargo install --path . 
```

## Help page

```
pts> pts.exe -h
Usage: pts.exe [OPTIONS]

Options:
  -f, --filename <FILENAME>            [default: YOU_PICK_A_FILE]
  -r, --repl
  -v, --verbose
  -p, --ps-event
  -u, --utc
  -s, --sierror
  -l, --logoerror
  -i, --illegalevents <ILLEGALEVENTS>  [default: YOU_PICK_ILLEGAL_EVENTS]
  -a, --all
  -c, --csv <CSV>                      [default: YOU_PICK_A_CSV]
  -h, --help                           Print help
  -V, --version                        Print version
pts>
```
