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

```
cargo run -- -avf "hdplus_20240501_30866.pts" --fluid "uhd_fluid_database.csv" -c out.csv
```

```
pts -avf .\hdplus_20240514_31166.pts \
    -c out.csv \
    --fps 25 \
    --update-werbungen 
    --fluid 'C:\Users\sgraetz\OneDrive - CreateCtrl AG\uhd1-plannung\uhd_fluid_database.csv'
```
