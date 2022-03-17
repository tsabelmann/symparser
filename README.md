# symparser

[![Crates.io](https://img.shields.io/crates/v/symparser)](https://crates.io/crates/symparser)[![docs.rs](https://img.shields.io/docsrs/symparser)](https://docs.rs/crate/symparser/latest)![Crates.io](https://img.shields.io/crates/l/symparser)[![Crates.io](https://img.shields.io/crates/d/symparser)](https://crates.io/crates/symparser)

This crate implements a parser for the SYM file format, originally conceived by [PEAK-Systems](https://www.peak-system.com/). 
As of today, there exists a tool for Windows ([PCAN-Symbol Editor 6](https://www.peak-system.com/PCAN-Symbol-Editor-6.416.0.html#:~:text=Das%20von%20PEAK%2DSystem%20entwickelte,CAN%2DIDs%20zun%C3%A4chst%20Namen%20zugewiesen.))
that allows the creation and edition of SYM files. The crate is the foundation for a functional implementation of the 
SYM file format that will be implemented by the [cantools](https://github.com/tsabelmann/cantools-rs) crate. If you are looking
for software that can be used to analyse CAN-bus data using SYM, have a look at the Python package [cantools](https://github.com/cantools/cantools).

## License / Terms of Usage

The source code of this project is licensed under the MIT license. This implies that you are free to use, share, and adapt it. However, please give appropriate credit by citing the project.

## Contact

If you have problems using the software, find mistakes, or have general questions please use the [issue tracker](https://github.com/tsabelmann/symparser-rs/issues) to contact us.

## Contributors

- [Tim Lucas Sabelmann](https://github.com/tsabelmann)