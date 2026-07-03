// Package wickra provides idiomatic Go bindings for wickra-radar over its C ABI
// hub: build a Radar from a spec JSON, drive it with command JSON and read back
// the response JSON — the same protocol as the CLI and every other binding.
//
// The binding links the prebuilt C ABI library, staged per platform under
// ./lib/<goos>_<goarch>/, with the header vendored under ./include.
package wickra

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo linux,amd64 LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lwickra_radar -Wl,-rpath,${SRCDIR}/lib/linux_amd64
#cgo linux,arm64 LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lwickra_radar -Wl,-rpath,${SRCDIR}/lib/linux_arm64
#cgo darwin,amd64 LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lwickra_radar -Wl,-rpath,${SRCDIR}/lib/darwin_amd64
#cgo darwin,arm64 LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lwickra_radar -Wl,-rpath,${SRCDIR}/lib/darwin_arm64
#cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/windows_amd64 -l:wickra_radar.dll
#cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/windows_arm64 -l:wickra_radar.dll
#include <stdlib.h>
#include "wickra_radar.h"
*/
import "C"

import (
	"fmt"
	"runtime"
	"unsafe"
)

// Radar is a radar instance driven by JSON commands.
type Radar struct {
	handle *C.WickraRadar
}

// New builds a radar from a spec JSON string. Call Close when done (a finalizer
// also frees it, but explicit Close is preferred).
func New(specJSON string) (*Radar, error) {
	cspec := C.CString(specJSON)
	defer C.free(unsafe.Pointer(cspec))

	handle := C.wickra_radar_new(cspec)
	if handle == nil {
		return nil, fmt.Errorf("wickra-radar: invalid spec")
	}
	r := &Radar{handle: handle}
	runtime.SetFinalizer(r, (*Radar).Close)
	return r, nil
}

// Command applies a command JSON and returns the response JSON. It uses the C
// ABI's length-out protocol: a first call learns the length, then the response
// is read into a caller-owned buffer.
func (r *Radar) Command(cmdJSON string) (string, error) {
	ccmd := C.CString(cmdJSON)
	defer C.free(unsafe.Pointer(ccmd))

	n := C.wickra_radar_command(r.handle, ccmd, nil, 0)
	if n < 0 {
		return "", fmt.Errorf("wickra-radar: command failed (code %d)", int(n))
	}
	buf := make([]byte, int(n)+1)
	C.wickra_radar_command(
		r.handle,
		ccmd,
		(*C.char)(unsafe.Pointer(&buf[0])),
		C.size_t(len(buf)),
	)
	return string(buf[:n]), nil
}

// Close frees the radar handle. Safe to call more than once.
func (r *Radar) Close() {
	if r.handle != nil {
		C.wickra_radar_free(r.handle)
		r.handle = nil
	}
	runtime.SetFinalizer(r, nil)
}

// Version returns the library version.
func Version() string {
	return C.GoString(C.wickra_radar_version())
}
