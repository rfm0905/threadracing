# Thread Racing

- was reviewing concurrency and wanted to make the threads race 
- first one uses a barrier, second one just interleaves them. avoids the `println!` lock by acquiring and writing/reading the `Rwlock` in a snapshot scope. Race3 and 4 are the same as 1/2 but without the snapshot.
- barrier approach seems to be the best 
- def has some memory leaks or smth
