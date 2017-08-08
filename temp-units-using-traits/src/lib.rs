use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Temperature {
    Celcius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    fn to_celcius(self) -> Temperature {
        match self {
            c @ Temperature::Celcius(_) => c,
            Temperature::Fahrenheit(f) => Temperature::Celcius((f-32.0)*(5.0/9.0)),
            Temperature::Kelvin(k) => Temperature::Celcius(k-273.15),
        }
    }

    fn to_fahrenheit(self) -> Temperature {
        match self {
            Temperature::Celcius(c) => Temperature::Fahrenheit((c*(9.0/5.0))+32.0),
            f @ Temperature::Fahrenheit(_) => f,
            Temperature::Kelvin(k) => Temperature::Fahrenheit((k*(9.0/5.0))-459.67),
        }
    }

    fn to_kelvin(self) -> Temperature {
        match self {
            Temperature::Celcius(c) => Temperature::Kelvin(c+273.15),
            Temperature::Fahrenheit(f) => Temperature::Kelvin((f+459.67)*(5.0/9.0)),
            k @ Temperature::Kelvin(_) => k,
        }
    }
}

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Temperature) -> Self::Output {
        match (self, rhs) {
            (Temperature::Celcius(a), b @ _) => match b.to_celcius() {
                 Temperature::Celcius(b) => Temperature::Celcius(a + b),
                 _ => unreachable!(),
            },
            (Temperature::Fahrenheit(a), b @ _) => match b.to_fahrenheit() {
                Temperature::Fahrenheit(b) => Temperature::Fahrenheit(a + b),
                _ => unreachable!(),
            },
            (Temperature::Kelvin(a), b @ _) => match b.to_kelvin() {
                Temperature::Kelvin(b) => Temperature::Kelvin(a + b),
                _ => unreachable!(),
            }
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Temperature::Celcius(c) => write!(formatter, "{}°C", c),
            Temperature::Fahrenheit(f) => write!(formatter, "{}°F", f),
            Temperature::Kelvin(k) => write!(formatter, "{}K", k),
        }
    }
}

#[test]
fn eq_test() {
    assert_eq!(Temperature::Celcius(100.0), Temperature::Celcius(100.0));
    assert_eq!(Temperature::Fahrenheit(212.0), Temperature::Fahrenheit(212.0));
    assert_eq!(Temperature::Kelvin(373.15), Temperature::Kelvin(373.15));
}

#[test]
fn add_celcius_test() {
    assert_eq!(Temperature::Celcius(100.0) + Temperature::Celcius(100.0), Temperature::Celcius(200.0));
    assert_eq!(Temperature::Celcius(100.0) + Temperature::Fahrenheit(212.0), Temperature::Celcius(200.0));
    assert_eq!(Temperature::Celcius(100.0) + Temperature::Kelvin(373.15), Temperature::Celcius(200.0));
}

#[test]
fn add_fahrenheit_test() {
    assert_eq!(Temperature::Fahrenheit(212.0) + Temperature::Celcius(100.0), Temperature::Fahrenheit(424.0));
    assert_eq!(Temperature::Fahrenheit(212.0) + Temperature::Fahrenheit(212.0), Temperature::Fahrenheit(424.0));
    assert_eq!(Temperature::Fahrenheit(212.0) + Temperature::Kelvin(373.15), Temperature::Fahrenheit(423.99999999999994));
}

#[test]
fn add_kelvin_test() {
    assert_eq!(Temperature::Kelvin(373.15) + Temperature::Celcius(100.0), Temperature::Kelvin(746.3));
    assert_eq!(Temperature::Kelvin(373.15) + Temperature::Fahrenheit(212.0), Temperature::Kelvin(746.3));
    assert_eq!(Temperature::Kelvin(373.15) + Temperature::Kelvin(373.15), Temperature::Kelvin(746.3));
}

#[test]
fn to_celcius_test() {
    assert_eq!(Temperature::Celcius(100.0).to_celcius(), Temperature::Celcius(100.0));
    assert_eq!(Temperature::Fahrenheit(212.0).to_celcius(), Temperature::Celcius(100.0));
    assert_eq!(Temperature::Kelvin(373.15).to_celcius(), Temperature::Celcius(100.0));
}

#[test]
fn to_fahrenheit_test() {
    assert_eq!(Temperature::Celcius(100.0).to_fahrenheit(), Temperature::Fahrenheit(212.0));
    assert_eq!(Temperature::Fahrenheit(212.0).to_fahrenheit(), Temperature::Fahrenheit(212.0));
    assert_eq!(Temperature::Kelvin(373.15).to_fahrenheit(), Temperature::Fahrenheit(211.99999999999994));
}

#[test]
fn to_kelvin_test() {
    assert_eq!(Temperature::Celcius(100.0).to_kelvin(), Temperature::Kelvin(373.15));
    assert_eq!(Temperature::Fahrenheit(212.0).to_kelvin(), Temperature::Kelvin(373.15000000000003));
    assert_eq!(Temperature::Kelvin(373.15).to_kelvin(), Temperature::Kelvin(373.15));
}

#[test]
fn format_test() {
    assert_eq!(format!("{}", Temperature::Celcius(100.0)), "100°C");
    assert_eq!(format!("{}", Temperature::Fahrenheit(212.0)), "212°F");
    assert_eq!(format!("{}", Temperature::Kelvin(373.15)), "373.15K");
}
