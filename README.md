# Leftmove

This program aims to provide an automated polling of [Rightmove](https://www.rightmove.co.uk)[^1] for properties that meet the deisred specification.

[^1] Rightmove is the main property listing website in the UK.

## Design

- User sets search property criteria, e.g., price, location, date listed etc.
- Program searches rightmove website for properties that meet the criteria
- Properties that meet criteria are stored in DB
- If property is already in DB don't do anything
- If property not been added before, email the user the property details


