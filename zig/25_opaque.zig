// its like a secret box
// used for libs
// lib give me some type, and functions
// i cant create an instance by self, only with funcs in lib
// i can do anything by self, only with funcs from lib

const Window = opaque {};
const Button = opaque {};

extern fn show_window(*Window) callconv(.C) void;

test "opaque" {
    const main_window: *Window = undefined;
    show_window(main_window);

    const ok_button: *Button = undefined;
    show_window(ok_button); // here will be error
}

// OPAQUE WITH METHODS
const Window2 = opaque {
    fn show(self: *Window2) void {
        show_window2(self);
    }
};

extern fn show_window2(*Window2) callconv(.C) void;

test "opaque with declarations" {
    var main_window: *Window2 = undefined;
    main_window.show();
}
