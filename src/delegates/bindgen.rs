/* automatically generated by rust-bindgen 0.60.1 */

extern "C" {
    #[doc = " @brief Initializes the pico delegates implementation"]
    #[doc = ""]
    #[doc = " @warning This function must be called before any other delegate function is used"]
    pub fn pico_delegates_init();
}
extern "C" {
    #[doc = " @brief A buffer to write a panic message into"]
    pub static mut pico_panic_buf: [u8; 256usize];
}
extern "C" {
    #[doc = " @brief Calls the SDK panic handler"]
    #[doc = ""]
    #[doc = " @param message The panic message"]
    pub fn pico_panic();
}
extern "C" {
    #[doc = " @brief Gets a char from stdin"]
    #[doc = ""]
    #[doc = " @param result The target pointer"]
    pub fn pico_stdio_getc(result: *mut u8);
}
extern "C" {
    #[doc = " @brief Writes a char to stdout"]
    #[doc = ""]
    #[doc = " @param value The char to write"]
    pub fn pico_stdio_putc(value: u8);
}
extern "C" {
    #[doc = " @brief Sleeps the given amount of milliseconds"]
    #[doc = ""]
    #[doc = " @param ms The amount of milliseconds to sleep"]
    pub fn pico_sleep_ms(ms: u32);
}
extern "C" {
    #[doc = " @brief Sleeps the given amount of microseconds"]
    #[doc = ""]
    #[doc = " @param us The amount of milliseconds to sleep"]
    pub fn pico_sleep_us(us: u32);
}
extern "C" {
    #[doc = " @brief Launches some code function on core 1"]
    #[doc = ""]
    #[doc = " @param entry The entry point"]
    pub fn pico_core1_start(entry: ::core::option::Option<unsafe extern "C" fn()>);
}
extern "C" {
    #[doc = " @brief Stops core 1"]
    pub fn pico_core1_halt();
}
extern "C" {
    #[doc = " @brief Locks the shared spinlock"]
    #[doc = ""]
    #[doc = " @param irq A pointer store the current interrupt states"]
    pub fn pico_spinlock_lock(irq: *mut u32);
}
extern "C" {
    #[doc = " @brief Unlocks the shared spinlock"]
    #[doc = ""]
    #[doc = " @param irq The interrupt states to restore"]
    pub fn pico_spinlock_unlock(irq: u32);
}
extern "C" {
    #[doc = " @brief Starts a state machine"]
    #[doc = ""]
    #[doc = " @param pio The PIO index"]
    #[doc = " @param sm The state machine index"]
    #[doc = ""]
    #[doc = " @warning Please your C code is responsible to initialize the PIOs their state machines as appropriate"]
    pub fn pico_piosm_start(pio: u32, sm: u32);
}
extern "C" {
    #[doc = " @brief Stops a state machine"]
    #[doc = ""]
    #[doc = " @param pio The PIO index"]
    #[doc = " @param sm The state machine index"]
    #[doc = ""]
    #[doc = " @note It is intentional that this library does not provide a start function because PIO initialization is extremely"]
    #[doc = "       implementation dependant. You need to implement and invoke your initializer yourself."]
    pub fn pico_piosm_halt(pio: u32, sm: u32);
}
extern "C" {
    #[doc = " @brief Gets a value from a state machine queue"]
    #[doc = ""]
    #[doc = " @param result The target pointer"]
    #[doc = " @param pio The PIO index"]
    #[doc = " @param sm The state machine index"]
    #[doc = " @return `0` on success or `-1` in case of an error"]
    pub fn pico_piosm_get(result: *mut u32, pio: u32, sm: u32);
}
extern "C" {
    #[doc = " @brief Puts a value to a state machine queue"]
    #[doc = ""]
    #[doc = " @param pio The PIO index"]
    #[doc = " @param sm The state machine index"]
    #[doc = " @param value The value"]
    pub fn pico_piosm_put(pio: u32, sm: u32, value: u32);
}
extern "C" {
    #[doc = " @brief Initializes a GPIO pin"]
    #[doc = ""]
    #[doc = " @param pin The pin to initialize"]
    #[doc = " @param direction The pin direction (i.e. `0` to read, `1` to write)"]
    pub fn pico_gpio_init(pin: u32, direction: u8);
}
extern "C" {
    #[doc = " @brief Gets the state of a GPIO pin"]
    #[doc = ""]
    #[doc = " @param value The target pointer (i.e. `0` for low, `1` to high)"]
    #[doc = " @param pin The pin to read"]
    pub fn pico_gpio_get(value: *mut u8, pin: u32);
}
extern "C" {
    #[doc = " @brief Sets the state of a GPIO pin"]
    #[doc = ""]
    #[doc = " @param pin The pin to write"]
    #[doc = " @param value The value to set (i.e. `0` for low, `1` to high)"]
    pub fn pico_gpio_put(pin: u32, value: u8);
}