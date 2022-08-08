# Process Mining Rust
This is a Process Mining Rust Library focussed on the new OCEL data format. The repository will be **very unstable** for now as I am very time-bound and need to implement many features. This readme will be further updated in the future when I have more time. For now, here is a list of features that it currently contains:

## Objects
- Object-Centric Event Log (OCEL): 
	- jsonocel importing and exporting with RFC-3339 compliant datetime.
	- jsonocel validation
- Object-Centric Directed Graph (OCDG):
	- generation using an OCEL
	- Importing and exporting to gexf (gexfocdg) file format
- Object Linking - Link object ids and event ids between objects

## Feature Extraction
- OCEL/OCDG:
	- Object Point Features
	- Object Group Features
	- Event Point Features
	- Event Group Features
	- Event Situation Targets
	- Object Situation Targets
	- Timeseries


