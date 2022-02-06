package car

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

// Custom vehicle types.
type CarOrigin string
type Acceleration int
type Displacement uint16

// Date struct that is used in car struct.
type CarDate struct {
	time.Time
}

// Car struct has field names and types relevant to JSON data,
// however, not every field can be mapped using `json.Unmarshal` method,
// therefore some fields have custom type, struct or/and method for parsing it.
// This structure standardizes non-standardized JSON input.
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

// Get integer value from string.
func parseInt(value string) int {
	re, _ := regexp.Compile(`[^\d]`)
	replaced := re.ReplaceAllString(value, "")
	v, _ := strconv.Atoi(replaced)
	return v
}

// Unmarshal function for acceleration field type.
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

// Unmarshal function for car origin field type.
func (val *CarOrigin) UnmarshalJSON(b []byte) error {
	var s string
	json.Unmarshal(b, &s)
	origin := CarOrigin("Country: " + s)
	*val = origin
	return nil
}

// Unmarshal function for year field type.
func (val *CarDate) UnmarshalJSON(b []byte) error {
	var unparsed string
	json.Unmarshal(b, &unparsed)
	layout := "2006-01-02"
	t, _ := time.Parse(layout, unparsed)
	val.Time = t
	return nil
}

// Unmarshal function for displacement.
// The type of values for this field might be an quoted and unquoted number.
// This function parses bytes into string and then continue parsing into int value,
// If this operiation is successful the result will be assigned into the given field.
func (val *Displacement) UnmarshalJSON(b []byte) error {
	cleanValue := strings.Replace(string(b), "\"", "", 2)
	parsedInt, err := strconv.ParseInt(cleanValue, 10, 64)
	if err != nil {
		return nil
	}
	*val = Displacement(parsedInt)
	return nil
}

// Read file conent by given path.
func ReadContent(path string) ([]byte, error) {
	file, fileError := os.Open(path)
	if fileError != nil {
		log.Print("Unable to open file")
		return nil, errors.New("FileNotAccessible")
	} else {
		defer file.Close()
	}
	content, _ := ioutil.ReadAll(file)

	return content, nil
}

// Parse conent JSON content.
func parseContent(content []byte) []Car {
	var cars []Car
	err := json.Unmarshal(content, &cars)
	if err != nil {
		log.Printf("Parsing content error %v", err)
		return cars
	}
	return cars
}

// Sort by year, horsepower and name respectively.
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

// Create struct, format field values, parse them into specific
// types and sort them.
func makeCars(content []byte) *[]Car {
	cars := parseContent(content)
	sortContent(&cars)
	return &cars
}

// Parse items from given path.
func ParseFromFile(path string) *[]Car {
	content, err := ReadContent(path)
	if err != nil {
		return nil
	}
	cars := makeCars(content)
	return cars
}

// Parse json data given as a string.
func ParseJSON(jsonData string) string {
	cars := makeCars([]byte(jsonData))
	jsonCars, err := json.Marshal(cars)
	if err != nil {
		log.Printf("Marshal error (empty string) %v", err)
		return ""
	}
	return string(jsonCars)
}
