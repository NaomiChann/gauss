# Gaussian Elimination
`Anna Gabriele Marques de Oliveira` `Naomi Celestino Ribes`  
- [PDF](../master/gauss.pdf)    
- [Original C implementation](https://github.com/gmendonca/gaussian-elimination-pthreads-openmp)
```
cd gauss
gcc gauss.c -o gauss
./gauss <matrix_dimension> [seed]
```
- Rust implementation
```
cd gausst
cargo build
cargo run <matrix_dimension> [seed]
```
- Go implementation
```
cd gouss
go run <matrix_dimension> [seed]
```