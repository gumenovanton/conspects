const std = @import("std");

test "catch memory errors" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var debug = std.heap.DebugAllocator.init(gpa.allocator());
    defer {
        debug.deinit(); // Проверит утечки!
        _ = gpa.deinit();
    }

    const allocator = debug.allocator();

    // Тестируем проблемный код
    const ptr = try allocator.alloc(u8, 10);

    // ❌ Умышленно делаем ошибку
    allocator.free(ptr);
    // ptr[0] = 1;  // Раскомментировать для use-after-free
    // allocator.free(ptr);  // Раскомментировать для double-free

    // Если есть ошибки - тест упадёт с понятным сообщением!
}

// WRAPPER FOR DEBUG
// checks use after free
// double free
// memory leaks
// out of bounds

// WHEN TO USE
// tests
// debuging with difficult bugs
// in CI/CD
// when dev libs
// code review for other coder

// DO NOT USE IN PRODUCTION
