# FastLAS

A Rust implementation of the LAStools.

### Speed
Compared parsing and writing a 7028703 point cloud

- FastLAS: 
```
Total time: 0:04.81
User Mode (s) 3.78
Kernel Mode (s) 1.00
CPU: 99%
```

- LAStools:
```
Total time: 0:06.90
User Mode (s) 6.58
Kernel Mode (s) 0.15
CPU: 97%
```
