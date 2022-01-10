# Earthquake Epicenters

To figure out how far away an earthquake is you just need one seismograph but to pinpoint where the earthquake happened, it's epicenter, you need three seismographs in three different locations. Say an earthquake occurs at some unknown location `(x0, y0)` on a 2D plane. It will emit seismic waves which travel through the Earth and are detected by seismograph stations. More than one type of wave is emitted but we'll just consider the faster compressional P-waves which travel at around `v = 6km/s` and arrive at the seismographs first.

The P-waves are faster and arrive at the seismograph before the shearing S-waves. They are both types of elastic body waves as they travel through the Earth with the P-waves being longitudinal while the S-waves being transverse. You can also think of the P and S as standing for primary and secondary. The surface waves are the slowest of the three and do the most damange as evidenced by the high ground motion registered on the seismograph. 

## Task
Given the position of three seismographs `(xi, yi)` and arrival time of the seismic waves at each seismograph `ti` where `i = 1, 2, 3` as inputs, determine and return the earthquake's epicenter `(x0, y0)`.

Input: `(xi, yi)`  in kilometers where `-100 < x, y < 100` and `ti` in seconds for `i = 1, 2, 3` 
Output: The earthquake's epicenter `(x0, y0)` in kilometers.