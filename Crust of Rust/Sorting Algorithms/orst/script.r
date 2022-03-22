t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# to plot # comparisons
ggplot(t, aes(n, comparisions, colour = algorithm)) + geom_point() + scale_y_log10()
# to plot runtime
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + scale_y_log10()
# add line
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + geom_line() + scale_y_log10()