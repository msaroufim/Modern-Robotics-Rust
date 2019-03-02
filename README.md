# Modern Robotics Library in Rust

This is a port of the code included in the Modern Robotics class and textbook found [here](https://github.com/NxRLab/ModernRobotics). The code was ported until Chapter 5 which deal Forward Kinematics and Inverse Kinematics.

The code is heavily inspired by the [C port](https://github.com/WanlinYang/Modern-Robotics-C-Library) and uses [Corrode](https://github.com/jameysharp/corrode) to transform the C code into Rust. Corrode is mostly written in Haskell and the outputted cord works but is unfortunately 4 times longer than the original C file

## Why do robotics in Rust
I'm interested in this effort because Rust promises
* Performance which for real time applications like robots is crucial to achieve smooth animations 
* A robotics library written in Rust can be potentially faster than alternatives written in Python which means it's easier to train behaviors for things like reinforcement learning algorithms
* Safety which means the robot won't deal with potentially dangerous undefined behaviors at runtime

## Potential improvements
* Can use better Rust syntax to make the code shorter and more readable
* Port other chapters