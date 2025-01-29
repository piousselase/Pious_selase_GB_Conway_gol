# Pious_selase_GB_Conway_gol
A simple rust version of the famous conway game of life. This project is a command-line interface (CLI) version of the famous cellular automaton devised by mathematician John Conway. Written in Rust, it leverages the language's performance and safety features to deliver a fast and interactive simulation of life-like patterns in a grid-based environment.

**HOW TO PLAY**  
-> When prompted, enter the grid size (e.g., 10x10).  
-> Provide the coordinates of initial live cells (e.g., 1,1 2,2 3,3).  
-> Set the number of generations to simulate.  
-> Watch life evolve in your terminal!  

**RULES OF THE GAME**
Rules of the Game
-> Any live cell with fewer than two live neighbors dies (underpopulation).
-> Any live cell with two or three live neighbors survives to the next generation.
-> Any live cell with more than three live neighbors dies (overpopulation).
-> Any dead cell with exactly three live neighbors becomes a live cell (reproduction).

**HOW IT WORKS**
-> The game runs on a 2D grid, where each cell is either alive or dead.
-> The state of each cell is updated simultaneously in discrete time steps.
-> The evolution is determined entirely by the initial state, with no further input.

FIND ATTACHED ALL FILES AND AARCH64, LINUX AND X86_64 BINARIES 
