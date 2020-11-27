README.md

# gprfix
Cleans NEMA data collected for a GPR unit.

gprfix was developed to "clean" NEMA data collected for a GPR unit.

The executable takes no arguments. It simply:
1.	creates a subfolder named "Clean_DZGs"
2.	looks for all "DZG" files in the current folder
3.	loads each file one line at a time
4.	retains any line that starts with "GSSIS" or "GPGGA"
5.	removes duplicate "GSSIS" lines between "GPGGA" lines
6.	creates and writes a revised file with the same name into the "Clean_DZGs" folder

To do in the future:
1.	generalize the pattern it looks for
2.	allow files with other extensions to be loaded
3.	better error checking
