// ADC will be used to measure ambient temperature using a 10k Ohm (NTC) Thermistor
// The ADC collected value will be converted to temperature and sent to the terminal output
/********************************************************************************************
// 1.) Kick off the ADC and obtain a reading/sample
// 2.) Calculate the temperature in Celcius
// 3.) Print the temperature value on the terminal
// 4.) Wait for 0.5 seconds
// 5.) Go back to step 1
********************************************************************************************/
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::
{
    delay::Delay,
    prelude::*,
    gpio::{Io, Level, Output},
    analog::adc::{Adc, AdcConfig, Attenuation},
};
use esp_println::println;
use libm::log;

#[entry]
fn main() -> ! 
{
    // Take Peripherals & Configure Clocks
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create handle for ADC configuration parameters
    let mut adc_config = AdcConfig::new();

    // Configure ADC channel
    let mut adc_pin = adc_config.enable_pin(io.pins.gpio4, Attenuation::Attenuation11dB);

    // Create ADC Driver
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    // Configure output pin
    let mut led = Output::new(io.pins.gpio9, Level::High);

    // Initalize output pin to low
    led.set_low();

    let mut blinkdelay = 1000_u32;

    const B: f64 = 3950.0;
    const VMAX: f64 = 4095.0;
    loop 
    {
        // Get ADC reading
        let sample: f64 = (adc.read_oneshot(&mut adc_pin).unwrap()).into();

        let log_value = log(1.0 / ((VMAX / sample) - 1.0));

        // Convert to temperature
        // T_Kelvin = 1 / ( (1/B) * log(R_NTC / R_0) + (1/T0) )
        // T_Celsius = T_Kelvin - 273.15
        let temperature_K = 1.0 / (log_value / B + 1.0 / 298.15);

        let temperature_C = temperature_K - 273.15;

        // Print the temperature output
        println!("Temperature {:.02} Celcius\r", temperature_C);

        // Set the Delay of the LED based on the temperature reading
        // Lower the temperature slower the blink
        let blinkdelay = match temperature_C
        {
            t if t < -20.0 => 5000,
            t if t >= -20.0 && t < -10.0 => 4000,
            t if t >= -10.0 && t < 0.0 => 3000,
            t if t >= 0.0 && t < 10.0 => 2000,
            t if t >= 10.0 && t < 20.0 => 1000,
            t if t >= 20.0 && t < 30.0 => 500,
            t if t >= 30.0 && t < 40.0 => 250,
            t if t >= 40.0 && t < 60.0 => 100,
            t if t >= 60.0 => 50,
            _ => 500,
        };
        led.set_high();
        delay.delay_millis(blinkdelay);
        led.set_low();

        // Wait half a second before next sample
        delay.delay_millis(500_u32);
    }
}
