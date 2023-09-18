# pts
A pts file parser

## help page
```
pts> .\target\release\pts.exe -h
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
.\pts.exe -f hdplus_20230915_26886.pts  -v
2 sierrors
SiEvent: {
	title: Free Fenster
	eventid: HDPLUHD_4832103_SI
	serviceid: HDPLUHD
	programid: HDPLUHD_4832103
	startime:  2023-09-15T06:00:00.000Z
	endtime:   2023-09-15T12:32:00.000Z #<-- Time gap
	SiStandard: {
		displayed starttime: 2023-09-15T06:00:00.000Z
		displayed endtime: 2023-09-15T12:35:10.000Z #<-- Time overlap
	}
}
SiEvent: {
	title: Sendepause
	eventid: HDPLUHD_4844932_SI
	serviceid: HDPLUHD
	programid: HDPLUHD_4844932
	startime:  2023-09-15T12:35:00.000Z #<-- Time gap
	endtime:   2023-09-15T15:30:00.000Z
	SiStandard: {
		displayed starttime: 2023-09-15T12:35:00.000Z #<-- Time overlap
		displayed endtime: 2023-09-15T15:30:00.000Z
	}
}
```
