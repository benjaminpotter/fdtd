# Set the output to an SVG file
set terminal svg size 800,600 enhanced font 'Helvetica,10'

# Set labels and color palette
set xlabel "X-axis"
set ylabel "Y-axis"
set title "Heatmap for %s"
set pm3d map               # Use 2D heatmap projection
set palette defined ( 0 "blue", 1 "green", 2 "yellow", 3 "red" )

# Loop through each file and plot its heatmap
# Use a system call to list all files in the directory with a specific extension (e.g., *.dat)
# For each file, we plot the matrix data as a heatmap

files = system(sprintf("ls %s/*", DIR))
num_files = words(files)

# Loop over each file and plot
do for [i=1:num_files] {
    file_path = word(files, i)
    # Print which file is being plotted
    print "Plotting: " . file_path

    # Generate the heatmap for this file
    set output sprintf("~/downloads/%d.svg", i)  # Output file name for each heatmap
    set title sprintf("Heatmap for %s", file_path)  # Title includes the file name

    # Plot the matrix as a heatmap
    splot file_path matrix with image
}

