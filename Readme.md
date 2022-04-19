# J. Scott Thompson's PhD Specialty Exam Models
This package contains some simple "models" for measuring bespoke forms of software complexity.

The file [src/main.rs](src/main.rs) contains:
* A simple data structure for holding state vector data (position, velocity, orientation, and orientation rates)
* Three functions to update a state vector for a simulated platform (inspired by Weintraub's white paper in the docs folder)
  * 2DOF - the simulated platform can move in the horizontal XY plane (2 degrees of freedom)
  * 2DOF with turn rate constraints - the simulated platform has a limit on how quickly it can change its heading
  * 3DOF - the simulated platform can move in three dimensions and can change its heading and roll
* A main function that provides an example of an initial state vector and the results of applying all three of the above functions to that initial condition.

Execute this using the command `cargo run`.

The output has been saved and is stored in XLSX, PDF, and TXT format.

The output of the tool rust-code-analysis has been saved in [src_main.rs.toml](src_main.rs.toml). This contains a number of computed metrics, including cyclomatic complexity and cognitive complexity. The tool was envoked with the following parameters:
`.\rust-code-analysis-cli.exe -m -p src/main.rs -O toml --pr -o .`
