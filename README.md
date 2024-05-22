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

## Changelog
### 22.05.2024 - improved missing text view
```
PS C:\Users\
Missings texts:
|--------------------------------------------------------------------------------------------------------------------------|
| title                                              | progarmid       | start                   | end                     |
|--------------------------------------------------------------------------------------------------------------------------|
| ran SAT.1 Bundesliga: Countdown                    | P7S1UHD_4981038 | 27.05.2024 20:30:00.000 | 27.05.2024 21:10:00.000 |
|--------------------------------------------------------------------------------------------------------------------------|
| ran SAT.1 Bundesliga: 1. Halbzeit                  | P7S1UHD_4981039 | 27.05.2024 21:10:00.000 | 27.05.2024 22:00:00.000 |
|--------------------------------------------------------------------------------------------------------------------------|
| ran SAT.1 Bundesliga: Halbzeitanalyse              | P7S1UHD_4981040 | 27.05.2024 22:00:00.000 | 27.05.2024 22:15:00.000 |
|--------------------------------------------------------------------------------------------------------------------------|
| ran SAT.1 Bundesliga: Highlights                   | P7S1UHD_4981041 | 27.05.2024 22:15:00.000 | 27.05.2024 22:45:00.000 |
|--------------------------------------------------------------------------------------------------------------------------|
| ran SAT.1 Bundesliga: 2. Halbzeit                  | P7S1UHD_4981042 | 27.05.2024 22:45:00.000 | 27.05.2024 23:35:00.000 |
|--------------------------------------------------------------------------------------------------------------------------|
| title                                              | progarmid       | start                   | end                     |
|--------------------------------------------------------------------------------------------------------------------------|
Error Summary:
0   time errors
0   id errors
0   logo errors
0   special event errors
0   length errors
0   si length errors
0   vaerrors
0   sierrors
5   missing_texts
PS C:\Users\
```
