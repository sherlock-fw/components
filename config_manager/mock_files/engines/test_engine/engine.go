package main

import (
  "fmt"
  "flag"
)


func main(){

  search := flag.String("search","","search flag")
  flag.Parse()

  if *search == "user123"{
    fmt.Println("test output")
  }

}


