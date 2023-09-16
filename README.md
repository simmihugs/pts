# pts
A pts file parser

## Current status

currently a hard coded pts file is loaded, deserialized and the first
3 SiEvents are printed.

```
╭[simmi@xubu] ~/Projects/pts
╰─> cargo run                                                   on branch main
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/pts`
siEvent: {
	title: Free Fenster
	eventid: HDPLUHD_4832103_SI
	serviceid: HDPLUHD
	programid: HDPLUHD_4832103
	startime:  2023-09-15T06:00:00.000Z
	endtime:   2023-09-15T12:35:00.000Z
	SiStandard: {
		starttime: 2023-09-15T06:00:00.000Z
		endtime:   2023-09-15T12:35:00.000Z
	}
}
siEvent: {
	title: Sendepause
	eventid: HDPLUHD_4844932_SI
	serviceid: HDPLUHD
	programid: HDPLUHD_4844932
	startime:  2023-09-15T12:35:00.000Z
	endtime:   2023-09-15T15:30:00.000Z
	SiStandard: {
		starttime: 2023-09-15T12:35:00.000Z
		endtime:   2023-09-15T15:30:00.000Z
	}
}
siEvent: {
	title: Free Fenster
	eventid: HDPLUHD_4844933_SI
	serviceid: HDPLUHD
	programid: HDPLUHD_4844933
	startime:  2023-09-15T15:30:00.000Z
	endtime:   2023-09-15T18:00:00.000Z
	SiStandard: {
		starttime: 2023-09-15T15:30:00.000Z
		endtime:   2023-09-15T18:00:00.000Z
	}
}
╭[simmi@xubu] ~/Projects/pts
╰─> cargo run                                                   on branch main
```
