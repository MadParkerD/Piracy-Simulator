# Piracy Simulator

This project does basic simulation of the economy during the age of sail.
## What was built


There are two pieces of code used in this project. The first is the economy mod that provides definitions and implementations for the port and goods types. The port type is meant to represent a port during the age of sail, and tracks the various types of goods bought and sold during that time. Things that were implemented for ports are display, update, and getmax(which returns the most valuable good). The goods enum contains the types of goods that were bought and sold during the age of sail, and one value associated with those types. Things implemented for the goods enum are getVal, setVal, combine, swap, and same(which checks if two enums are the same type).

The second piece of code is the piracy portion of this project. This contains the declaration and implementation for ships, crew, health and wounds, along with their implementations. Ships contain crew, along with various points of data about the ship, and a hold of goods.Things implemented for ships include buying, selling and visiting ports, and corresponding functions for use with non-player ships. Crew objects contain a name and a health object containing wounds.Tho only thing implemented for health is a total function, what adds up all of the injuries it contains. Wounds is an enum that outlines the types of injuries a crew member can have, and values associated with them.



## How it works

The project initially loads a set of both ships and ports from a file, deserializing them from json. Ports are created from every line in places.txt with static connections to other ports in a vector, to simulate a graph, allowing ship objects to access routes to take. Ship objects are created for every item in ships.txt, and spawned in at a starting port. The player is then spawned at a starting port. The player is then presented with a menu, and asked to choose buying, selling, displaying, or visiting another port. If the player chooses to buy a menu pops up that asks them what to buy, and checks to see that they have enough money to buy what they entered. If they did their ships hold is updated with the bought items. If they choose to sell the same happens. If either of these functions are called, the ports update function is also called, which readjusts the ports prices based on what has been bought and sold. If the player chose to display then information about the port, as well as information about the other ships at the port will be displayed. If the player chooses to visit another port the ships visit function is called, updating the ships location. After the player completes the visit function a similar function is called for all of the npc ships, which each randomly choose which port from the list they can visit to go to. After this function completes the npc ships then sell the contents of their holds, and buy new items.

## What doesn't work

The original idea for this project had it being a multiple person endeavor, with one person writing the economy simulator and one person writing the piracy simulator. No second person was ever found, so only the core features of both were ever implemented. Though currently implemented, there is no way of doing anything with health or wounds, as combat was never implemented. The economy simulation that ports use is very shallow, with the updating of prices being decided randomly based on individual transactions, which there are never enough of to make use of. The npc ships used in this were never fleshed out fully, and the decisions they make about where to travel and what to buy and sell are random, with no weight system for choosing in an intelligent way. The planned feature that would allow a player to track costs, and sales was never implemented.

With this said, all currently implemented functions are working as intended with no issues, and are tested.

## Lessons Learned

The first thing I learned is that rust does not provide an easy or simple way of working with graphs. There are several crates that could have been used, but for the sake of time none were included. I also learned that the scope of a project is very important to outline before any final decision is made about end goal. It was simply not possible to implement all of the features I intended with the time I was able to work on this project, in part because rust requires much more effort to write in and in part because no second person was ever found to take on one half. Before this project I had never had the opportunity to use enums in any meaningful way, and learning to deal with the concept was enlightening. Were I to do this again I would either find a second person or choose a different project with a smaller scope. 

### Prerequisites

[Latest Rust Nightly](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html)

[Cargo](https://crates.io/)

### Installing

Clone all files and Cargo run

## Running the tests

Run cargo test in the top level directory.

## Built With
[Serdo Json](https://github.com/serde-rs/json)

## Authors

* **Madiosn Parker-Durost** - *All work*
