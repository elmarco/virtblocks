// Virt Blocks
//
// Copyright (C) 2019 Red Hat, Inc.
//
// This software is distributed under the terms of the MIT License.
// See the LICENSE file in the top level directory for details.

package main

import "C"

import (
	"github.com/virtblocks/virtblocks/c/go/pkg/objects"
	"github.com/virtblocks/virtblocks/go/native/pkg/devices"
)

//export devices_memballoon_new
func devices_memballoon_new() C.int {
	var goMemballoon = devices.NewMemballoon()
	return C.int(objects.DevicesMemballoonAdd(goMemballoon))
}

//export devices_memballoon_free
func devices_memballoon_free(cMemballoon C.int) {
	objects.DevicesMemballoonDel(int(cMemballoon))
}

//export devices_memballoon_set_model
func devices_memballoon_set_model(cMemballoon C.int, cModel C.int) {
	var goMemballoon = objects.DevicesMemballoonGet(int(cMemballoon))
	goMemballoon.SetModel(devices.MemballoonModel(cModel))
}

//export devices_memballoon_get_model
func devices_memballoon_get_model(cMemballoon C.int) C.int {
	var goMemballoon = objects.DevicesMemballoonGet(int(cMemballoon))
	return C.int(goMemballoon.Model())
}

//export devices_memballoon_to_string
func devices_memballoon_to_string(cMemballoon C.int) *C.char {
	var goMemballoon = objects.DevicesMemballoonGet(int(cMemballoon))
	return C.CString(goMemballoon.String())
}
