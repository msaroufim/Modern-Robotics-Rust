# Modern Robotics Library in Rust

This is a port of the code included in the Modern Robotics class and textbook found [here](https://github.com/NxRLab/ModernRobotics). The code was ported until Chapter 5 which deal Forward Kinematics and Inverse Kinematics.

The code is heavily inspired by the [C port](https://github.com/WanlinYang/Modern-Robotics-C-Library) and uses [Corrode](https://github.com/jameysharp/corrode) to transform the C code into Rust. Corrode is mostly written in Haskell and the outputted cord works but is unfortunately 4 times longer than the original C file