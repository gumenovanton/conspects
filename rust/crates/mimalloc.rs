MIMALLOC
// this is tha allocator that can allocate a buffer for every thread
// than mimalloc takes the memory from buffer instead of doing new allocations

ADD TO PROJECT
// add mimalloc via cargo
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    // Ваш код Axum/Postgres теперь будет использовать mimalloc
}
