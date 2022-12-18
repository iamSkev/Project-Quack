# Go

To run this Go file, first you need to install Go from [the official website](https://go.dev/doc/install), after that you'll need to either clone the repo and get the file, download the file itself or copy all of the source code.


After getting the file or source code, do this command in the directory where the source code of the spts calc is
```go
go mod init
```

after that it should make a `go.mod` file .

You will need to install a dependancy that the code needs which is the `exp/slices` module, you will need to run
```go
go get golang.org/x/exp/slices
```

It should now be able to run and you can run it with (note that your file should be named `spts_calc.go` if you copied the source code manually if you run this command)
```go
go run spts_calc.go
```