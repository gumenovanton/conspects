//>> FOR WHAT 
// to create and manipulate paths 

//>> FUNCS path, filepath
//<< Base() 
// get path base(last element) 
fmt.Println(path.Base("/a/b"))
fmt.Println(path.Base("/a/b.exe"))
fmt.Println(path.Base("/"))
fmt.Println(path.Base(""))
// b 
// b.exe 
// /
// . 

//<< Clean() 
// clear the path 
paths := []string{
    "a/c",
    "a//c",
    "a/c/.",
    "a/c/b/..",
    "/../a/c",
    "/../a/b/../././/c",
    "",
}

for _, p := range paths {
    fmt.Printf("Clean(%q) = %q\n", p, path.Clean(p))
}
// Clean("a/c") = "a/c"
// Clean("a//c") = "a/c"
// Clean("a/c/.") = "a/c"
// Clean("a/c/b/..") = "a/c"
// Clean("/../a/c") = "/a/c"
// Clean("/../a/b/../././/c") = "/a/c"
// Clean("") = "."

//<< Dir() 
// path dir 
fmt.Println(path.Dir("/a/b/c"))
fmt.Println(path.Dir("a/b/c"))
fmt.Println(path.Dir("/a/"))
fmt.Println(path.Dir("a/"))
fmt.Println(path.Dir("/"))
fmt.Println(path.Dir(""))
// /a/b
// a/b
// /a
// a
// /
// .

//<< Ext() 
// extention of file 
fmt.Println(path.Ext("/a/b/c/bar.css"))
// .css 

//<< IsAbs() 
// weather the path is absolute  
fmt.Println("On Unix:")
fmt.Println(filepath.IsAbs("/home/gopher"))
fmt.Println(filepath.IsAbs(".bashrc"))
fmt.Println(filepath.IsAbs(".."))
fmt.Println(filepath.IsAbs("."))
fmt.Println(filepath.IsAbs("/"))
fmt.Println(filepath.IsAbs(""))
// true
// false
// false
// false
// true
// false

//<< Join() 
// join elements in path 
fmt.Println(path.Join("a", "b", "c"))
fmt.Println(path.Join("a", "b/c"))
fmt.Println(path.Join("a/b", "c"))
fmt.Println(path.Join("a/b", "../../../xyz"))
fmt.Println(path.Join("", ""))
fmt.Println(path.Join("a", ""))
fmt.Println(path.Join("", "a"))
// a/b/c
// a/b/c
// a/b/c
// ../xyz
// 
// a
// a

//<< Match() 
// match path by syntax 
pattern:
	{ term }
term:
	'*'         matches any sequence of non-/ characters
	'?'         matches any single non-/ character
	'[' [ '^' ] { character-range } ']'
	            character class (must be non-empty)
	c           matches character c (c != '*', '?', '\\', '[')
	'\\' c      matches character c

character-range:
	c           matches character c (c != '\\', '-', ']')
	'\\' c      matches character c
	lo '-' hi   matches character c for lo <= c <= hi
    
fmt.Println(path.Match("abc", "abc"))
fmt.Println(path.Match("a*", "abc"))
fmt.Println(path.Match("a*/b", "a/c/b"))
// true <nil>
// true <nil>
// false <nil>

//<< Solit() 
// split path on parts  
split := func(s string) {
    dir, file := path.Split(s)
    fmt.Printf("path.Split(%q) = dir: %q, file: %q\n", s, dir, file)
}
split("static/myfile.css")
split("myfile.css")
split("")
// path.Split("static/myfile.css") = dir: "static/", file: "myfile.css"
// path.Split("myfile.css") = dir: "", file: "myfile.css"
// path.Split("") = dir: "", file: ""

//<< Localize() 
// converts slash separated path to os path style 
func Localize(path string) (string, error)

//<< ToSlash() 
// converts os path style to slash style 
func ToSlash(path string) string 

