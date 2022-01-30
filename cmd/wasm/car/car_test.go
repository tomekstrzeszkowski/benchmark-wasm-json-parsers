package car

import (
	"testing"
)

func TestStruct(t *testing.T) {
	val := parseInt("\"Abc12\"")
	if val != 12 {
		t.Errorf("got %q, wanted %q", val, 12)
	}
}
