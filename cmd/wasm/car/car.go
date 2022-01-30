package car

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

type CarOrigin string
type Acceleration int
type Displacement uint16

type CarDate struct {
	time.Time
}

type Car struct {
	Name             string  `json:",omitempty"`
	Miles_per_Gallon float64 `json:",omitempty"`
	Displacement     Displacement
	Horsepower       uint8
	Weight_in_lbs    uint16
	Cylinders        int
	Year             CarDate
	Origin           CarOrigin `json:",omitempty"`
	Acceleration     Acceleration
}

func parseInt(value string) int {
	re, _ := regexp.Compile(`[^\d]`)
	replaced := re.ReplaceAllString(value, "")
	v, _ := strconv.Atoi(replaced)
	return v
}

// Unmarshal function for acceleration field type
func (val *Acceleration) UnmarshalJSON(b []byte) error {
	var s int
	json.Unmarshal(b, &s)
	// check if real zero
	if s == 0 && string(b) != "0" {
		value := parseInt(string(b))
		s = value
	}
	acceleration := Acceleration(s)
	*val = acceleration
	return nil
}

// Unmarshal function for car origin field type
func (val *CarOrigin) UnmarshalJSON(b []byte) error {
	var s string
	json.Unmarshal(b, &s)
	origin := CarOrigin(s + "FF")
	*val = origin
	return nil
}

// Unmarshal function for year field type
func (val *CarDate) UnmarshalJSON(b []byte) error {
	var unparsed string
	json.Unmarshal(b, &unparsed)
	layout := "2006-01-02"
	t, _ := time.Parse(layout, unparsed)
	val.Time = t
	return nil
}

//Unmarshal function for displacement.
//The type of values for this field might be an quoted and unquoted number.
//This function parses bytes into string and then parses it into int value,
//If this operiation is successful the result will be assigned into the given field.
func (val *Displacement) UnmarshalJSON(b []byte) error {
	cleanValue := strings.Replace(string(b), "\"", "", 2)
	parsedInt, err := strconv.ParseInt(cleanValue, 10, 64)
	if err != nil {
		return nil
	}
	*val = Displacement(parsedInt)
	return nil
}

func ReadContent() []byte {
	file, fileError := os.Open("/home/t/Desktop/wasm/assets/cars.json")
	if fileError != nil {
		fmt.Println(fileError)
	} else {
		defer file.Close()
	}
	content, _ := ioutil.ReadAll(file)

	return content
}

func parseContent(content []byte) []Car {
	var cars []Car
	err := json.Unmarshal(content, &cars)
	if err != nil {
		fmt.Println(err)
	}
	return cars
}

//Sort by year, horsepower and name respectively
func sortContent(cars *[]Car) {
	sort.SliceStable(*cars, func(i, j int) bool {
		item1, item2 := &(*cars)[i], &(*cars)[j]
		if item1.Year.Time.Equal(item2.Year.Time) {
			if item1.Horsepower == item2.Horsepower {
				return item1.Name < item2.Name
			}
			return item1.Horsepower < item2.Horsepower
		}
		return item1.Year.Time.Before(item2.Year.Time)
	})
}

func MakeCars() {
	content := ReadContent()
	cars := parseContent(content)
	sortContent(&cars)

	for _, car := range cars {
		fmt.Println(
			"Cylinders", car.Cylinders,
			"Name", car.Name,
			"Displacement", car.Displacement,
			"Weight_in_lbs", car.Weight_in_lbs,
			"Horsepower", car.Horsepower,
			"Origin", car.Origin,
			"acc", car.Acceleration,
			"Miles_per_Gallon", car.Miles_per_Gallon,
			"Year", car.Year,
		)
	}
}

//TODO add tests :)
