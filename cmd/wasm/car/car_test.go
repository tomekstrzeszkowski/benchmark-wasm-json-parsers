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

func TestParsingFromFile(t *testing.T) {
	cars := ParseFromFile("testdata/cars.json")
	var previousCar *Car
	for i, car := range *cars {
		if i == 0 {
			str := car.Year.GoString()
			if car.Year.GoString() != "time.Date(1, time.January, 1, 0, 0, 0, 0, time.UTC)" {
				t.Error("The first car shall have undefined year", str)
			}
		}
		if previousCar != nil && previousCar.Year.Time.After(car.Year.Time) {
			t.Error("Cars should be sorted by date.")
		}
		previousCar = &car
	}
	lastCar := &(*cars)[len(*cars)-1]
	if lastCar.Year.GoString() != "time.Date(2022, time.January, 1, 0, 0, 0, 0, time.UTC)" {
		t.Error("Teh last car shall be the newer one", lastCar.Year.GoString())
	}
}
