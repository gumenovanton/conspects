const std = @import("std");

pub fn main() !void {
    // 1. Создаём GPA
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(); // Важно! Проверяет утечки

    // 2. Берём аллокатор
    const allocator = gpa.allocator();

    // 3. Используем для всего
    const string = try allocator.dupe(u8, "Hello");
    const numbers = try allocator.alloc(i32, 100);
    const obj = try allocator.create(u64);

    // 4. Освобождаем (или позволим GPA найти утечки при deinit)
    allocator.free(string);
    allocator.free(numbers);
    allocator.destroy(obj);
}

// DESCRIPTION
// can do enything
// speed and efficiency balance
// can detect memory leaks

// WHEN TO USE
// simple programs 90% of all apps
// libs
// when do not know what to use

// SMP allocator
// this is a general purpuse alocator designed for maximum preformance
// less sefty features
test "SmpAllocator" {
    const allocator = std.heap.smp_allocator;

    const bytes = try allocator.alloc(u8, 100);
    defer allocator.free(bytes);
}
