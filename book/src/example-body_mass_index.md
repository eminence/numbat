<!-- This file is autogenerated! Do not modify it -->

# Body mass index

``` numbat
# This script calculates the Body Mass Index (BMI) based on
# the provided mass and height values.

unit BMI: Mass / Length^2 = kg / m^2

fn body_mass_index(mass: Mass, height: Length) =
    mass / height² -> BMI

print(body_mass_index(70 kg, 1.75 m))
```