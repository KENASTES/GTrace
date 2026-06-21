#RFC: GTrace parser software 

** Author **             KENASTES
** Modify Date **        21/6/2026
** Status **             Implemented
 
 ---

 ## 1. Porpose and design of development
 This project is development for my personal Custom CNC laser engraver project that i develop for Microcontroller Subject at KMITL. I'll said about the Pain point that the laser engraver is used to
 solve is the point of self etching PCB is a really hard and detailed work that cost a lot of time and 
 cost. Althougt the 2 layer of self etching PCB is seem really hard to made so the porpose of this mechine
 is to make a self made PCB become a lot of easier. But come to say that the mechine is drive by grbl laser 
 firmware so it operate as a CNC mechine so it only operate with G-code file but the PCB editor or PCB
 custom software are usually export a gerber file out of schematic draw. So the porpose of this software
 is to convert the Gerber out file to G-code for the CNC laser engraver to operate.

 ## 2. Goal
 The goal of this software development is to imprement the software which meant to be the file converter
 but still can modify the outcome for the varius use for laser engraver.
 * Gerber to gcode converter /
 * Deploy software to be the Open Source /
 * Picture scanning or engrave mode X

 ## 3. Architecture
 I choose the Rust to be the core engine instead of c/c++ and use the C#/.NET framework that because i just wanna learn the new tech stack that it so begin with the core design of the program as we know the gerber file certain a pattern of line to line position of trace and pin of PCB so figure out the pattern and engineer the Gtrace software to match the porpose and the dependant between the hardware and software