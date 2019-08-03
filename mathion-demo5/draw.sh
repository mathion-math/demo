set terminal png
set output './graph.png'

set autoscale

set title "Prey & Predation Model"
set style data linespoints

plot "a.dat" using 1:2 title "Prey", \
     "b.dat" using 1:2 title "Predation", \
