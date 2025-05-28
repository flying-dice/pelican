# Option 1: Use `dofile` to Load Pelican Before Sanitization

To use Pelican in the MSE, you will need to load your lua script using `dofile` **before** DCS sanitizes the
environment.

This file is located in the `Scripts` folder of your DCS World installation, for example:

{@includeCode ./MissionScripting.lua}