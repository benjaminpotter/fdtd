
set terminal gif animate delay 20 loop 0
set output sprintf("plots/gif/%s.gif", DIR)

# Set labels and color palette
set xlabel "x"
set ylabel "y"
set title "e-field"
set pm3d map               # Use 2D heatmap projection
set palette defined ( 0 "blue", 1 "green", 2 "yellow", 3 "red" )

# Loop through each file and plot its heatmap
# Use a system call to list all files in the directory with a specific extension
# For each file, we plot the matrix data as a heatmap

files = system(sprintf("ls data/%s/*", DIR))
num_files = words(files)

# Loop over each file and plot
do for [i=1:num_files] {
    file_path = word(files, i)

    # Print which file is being plotted
    print "Plotting: " . file_path

    # Plot the matrix as a heatmap
    splot file_path matrix with image
}

