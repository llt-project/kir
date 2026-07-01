package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: kir/target/release/libkir.dylib
#include "kir/include/kir.h"
*/
import "C"
import (
	"unsafe"
)

func main() {
	res := C.add(1, 2+5)
	C.print(res)

	strs := []string{"a", "b", "c"}

	cStrs := make([]*C.char, len(strs))

	for i, s := range strs {
		cStrs[i] = C.CString(s)
		defer C.free(unsafe.Pointer(cStrs[i]))
	}

	ptr := (**C.char)(unsafe.Pointer(&cStrs[0]))
	C.project_new(ptr, C.size_t(len(strs)))
}
