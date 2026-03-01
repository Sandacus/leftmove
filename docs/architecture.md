# Architecture

Leftmove is a program that searches for properties that meet deisred criteria. 
If properties that are found have not been seen before, then the new properties are saved and an email is sent with property link(s).

The property search, save and update cycle will be performed periodically.

Requirements;

1. Desired property criteria are supplied
2. Leftmove program sends a `GET` request to Rightmove with the desired property parameters
3. Response is parsed for properties that meet the search criteria
4. If not seen before, 
  1. properties are saved
  2. email sent with links to new properties
5. Steps 1 - 4 are repeated periodically