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

## Basic usage

```
pts> pts.exe -s -f file.pts -a -v
0   sierrors
|------------------------------------------------------------------------------------------------------------------------------|
| title        | programid | start                   | end                     | duration     | contentid            | logo    |
|--------------+-----------+-------------------------+-------------------------+--------------+----------------------+---------|
| Dranbleiben  | D_4844934 | 15.09.2023 14:35:00.000 | 15.09.2023 14:47:21.000 | 00:12:21.000 | cb7a119f84cb7b117b1b |         |
| Black        | D_4855219 | 15.09.2023 14:47:21.000 | 15.09.2023 14:47:24.000 | 00:00:03.000 | e90dfb84e30edf611e32 |         |
| NK_Maschine  | D_4855220 | 15.09.2023 14:47:24.000 | 15.09.2023 14:47:31.000 | 00:00:07.000 | b1735b7c5101727b3c6c |         |
| Geheimnisse  | D_4844935 | 15.09.2023 14:47:31.000 | 15.09.2023 15:18:32.400 | 00:31:01.400 | 5a2d6391e984c539d7b0 | LOGO_14 |
| Werbung      | D_4855221 | 15.09.2023 15:18:32.400 | 15.09.2023 15:24:02.400 | 00:05:30.000 | UHD1_WERBUNG-01      |         |
| Geheimnisse  | D_4855222 | 15.09.2023 15:24:02.400 | 15.09.2023 15:38:53.680 | 00:14:51.280 | 5a2d6391e984c539d7b0 | LOGO_14 |
|------------------------------------------------------------------------------------------------------------------------------|
```

