# Set the labels for the axes
set xlabel "X Axis"
set ylabel "Y Axis"

# Set the title of the plot
set title "Heatmap Example"

# Define the color palette for the heatmap
set palette defined ( 0 "blue", 1 "green", 2 "yellow", 3 "red")

# Set the range of the color scale
set cbrange [0:1]

# Add a colorbar (optional)
set colorbox

# Use 'with image' to plot the data as a heatmap
plot 'data/heatmap.dat' matrix with image

# Pause to keep the window open (optional, for interactive mode)
pause -1 "Press any key to continue\n"
