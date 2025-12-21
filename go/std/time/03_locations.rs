//>> ЛОКАЦИИ
//<< LoadLocation(name)
// создает time.Location по строке локации
saratov, _ := time.LoadLocation("Europe/Saratov")
fmt.Println(saratov) // Europe/Saratov

//<< LoadLocationFromTZData(name, data)
// нахуй

//<< FixedZone(name, offset)
// создает time.Location по строке локации и смещение от указанного пояса
// name - название часового пояса
layout := "02 Jan 06 15:04"
date := "09 Jun 95 19:30"
local := time.FixedZone("Local", -5*60*60)
fmt.Println(saratov) // "Local"
timeParsed, _ := time.ParseInLocation(layout, date, local)
fmt.Println(timeParsed) //1995-06-09 19:30:00 -0500 Local