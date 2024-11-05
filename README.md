

### Visualizing Results
The simulator itself is not a graphical program. To reduce complexity, it just
computes the value of the E field for each node every timestep. This data is 
given to the user as a 2D matrix of floating point values. GNUPlot can be used
to visualize the field as a heatmap. Commonly used GNUPlot scripts are included
in the `plots` directory for getting started.

```
$> gnuplot plots/heatmap.gp
```
