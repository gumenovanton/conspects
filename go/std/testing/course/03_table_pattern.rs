//>> table driven test pattern
// use it when want to test many similar cases
//! always write tests in table driven styly, becaiuse it is easy to add test cases in the future

// example
func TestAdd(t *testing.T) {
// when we use slice of structs
// in it i must have cases data
	type args struct {
		x int
		y int
	}
	tests := []struct {
		name string     // name of a test
		args args       // args
		want int        // want
	}{
		{"add two positive", args{4,5}, 9},
		{"add two positive", args{4,2}, 6},
		{"add two positive", args{1,5}, 6},
		{"add two positive", args{7,5}, 12},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := Add(tt.args.x, tt.args.y); got != tt.want {
				t.Errorf("Add() = %v, want %v", got, tt.want)
			}
		})
	}
}