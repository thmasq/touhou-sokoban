# Touhou Sokoban! 

  This is a small project that I decided to make on my own to learn how to use the Rust programming language on a real release for people to enjoy. I followed some tutorials on the ggez game engine and forked some projects that were already using it correctly and this helped me learn quite a lot. 

  I intend to keep working on this project and update the old dependencies to their newest version soon™. The codebase will change a bit when the dependencies are updated, and some assets will be changed when I have enough knowledge to implement them correctly. 

   

# Instructions and how to run 

#### Windows 

Download the source code from the releases or from the repository itself if you want to run on the bleeding edge. Paste the windows binary inside that folder and run it. The binary must be in the root of the folder that contains the data and resources folders. You may delete the rest if you want to. Inside the data folder there is a file named game_data.json. It changes what level the game is going to load when you run the executable. Currently only level 1 and 2 are populated. 

#### Linux 

##### Alert 

This version of the ggez engine has a bug that will instantly crash the game when you hover the mouse over the game window on X11 (wayland is still untested). If the mouse is over the spot where the window will spawn, it will also crash. The only way to fix that is by updating the engine, but that is still a to-do. 

##### How to run 

Download the source code from the releases or from the repo itself if you want to run it on the bleeding edge. Paste the linux elf binary inside the folder and run it. The binary must be in the root of the folder containing the data and resources folders. You may delete the others if you are not interested in compiling the code yourself. Inside the data folder there is a file named game_data.json. It changes what level the game is going to load when it is run. Currently only level 1 and 2 are populated. 

#### Compiling it yourself 

You need rust and cargo installed to compile the game. It can be compiled by running the command "cargo build --release", and it will be compiled to ./target/release/ . If you want to cross compile it on Linux this might help https://wiki.archlinux.org/title/Rust#Windows 

# Credits 

#### Olivia 

I would like to thank Olivia Ifrim for her comprehensive guide on how to use the ggez game engine. It clarified and helped me learn how to use it, and why each function, method, enum or macro was being used. Do check out her repo on that. Her code examples and books are very nice. https://github.com/iolivia/rust-sokoban 

#### Thanh and Ly Nguyen 

I would also like to thank Thanh Nguyen and Ly Nguyen for sharing their source code that went in the same direction as Olivia's project, but implemented more features. I am a beginner both in Rust and in programming, so reading their code was essential for me to understand how features such as refreshing the map or reading the map files from a .json files are implemented. It can be considered the code base of this project, and I do intend to maintain it. Do check out their project: https://github.com/thanhqng1510/sokoban-rust 

#### Kazuki Kazuma 

Lastly, I would like to thank Kazuki Kazuma, my friend, for taking some time to make the beautiful assets used in this game. We do intend to keep working together to make more complex assets once I have the skill to implement them in the game. Do check out his works: https://www.artstation.com/kazukikazuma https://www.instagram.com/wiesevictor/ 
