# Architecture
## General flow
- Load/create spreadsheet
- While active:
	- Wait for command
	- Execute command
	- Evaluate cells
- Ask for save
- Act accordingly and exit
	

## Files
- **src/main.rs**: argument parsing and external error handling
- **src/lib/mod.rs**: main loop, command recognition
- **src/spreadsheet/mod.rs**: Spreadsheet struct and corresponding methods
- **src/cell/mod.rs**: Cell struct and corresponding methods
- **src/startup/mod.rs**: functions needed on startup, i.e. load file or create file
