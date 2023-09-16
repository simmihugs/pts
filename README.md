# pts
A pts file parser

## Current status

currently a hard coded pts file is loaded, deserialized and the first
3 SiEvents are printed.

```
╭[simmi@xubu] ~/Projects/pts
╰─> cargo run  -- -f hdplus_20230915_26886.pts  -v                            on branch main

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


╭[simmi@xubu] ~/Projects/pts
╰─> cargo run  -- -f hdplus_20230915_26886.pts  -v                            on branch main
```
