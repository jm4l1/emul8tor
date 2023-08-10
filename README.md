# Emul8tor

Implementation of [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) virtual machine.

### Dependencies

- SDL2  [installations here](https://wiki.libsdl.org/SDL2/Installation)

### Running
 
 Run with `cargo` passing in the name of the rom file.
 ```
 cargo run roms/LOGO
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/emul8tor roms/LOGO`
 ```

 ![Logo](resources/logo.png)

### Testing functionalities

Test roms can be found [here](roms/tests/). These roms are from [Chip8 Test Suite](https://github.com/Timendus/chip8-test-suite). Description of the expected behavior is provided in that project.

![Test Output](resources/tests.png)

### References

The below resources were very useful as references.

- [https://aquova.net/](https://aquova.net/chip8/chip8.pdf), this pdf and github project.
- [Chip-8 Design Specification](http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf)
- [Chip8 Test Suite](https://github.com/Timendus/chip8-test-suite) was great verifying op-code functionality.
